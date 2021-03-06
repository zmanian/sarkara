use seckey::Bytes;
use super::{ AeadCipher, DecryptFail };


/// Ascon.
///
/// # Example(encrypt/decrypt)
/// ```
/// # extern crate rand;
/// # #[macro_use] extern crate sarkara;
/// # fn main() {
/// use sarkara::aead::{ Ascon, AeadCipher };
///
/// let (pass, nonce) = (
///     rand!(Ascon::key_length()),
///     rand!(Ascon::nonce_length())
/// );;
/// let data = rand!(rand!(choose 0..1024));
/// let ciphertext = Ascon::new(&pass)
///     .with_aad(&nonce)
///     .encrypt(&nonce, &data);
/// let plaintext = Ascon::new(&pass)
///     .with_aad(&nonce)
///     .decrypt(&nonce, &ciphertext)
///     .unwrap();
/// assert_eq!(plaintext, &data[..]);
/// # }
/// ```
pub struct Ascon {
    key: Bytes,
    aad: Vec<u8>
}

impl AeadCipher for Ascon {
    fn new(key: &[u8]) -> Self {
        Ascon {
            key: Bytes::new(key),
            aad: Vec::new()
        }
    }

    #[inline] fn key_length() -> usize { 16 }
    #[inline] fn tag_length() -> usize { Self::key_length() }
    #[inline] fn nonce_length() -> usize { Self::key_length() }

    fn with_aad(&mut self, aad: &[u8]) -> &mut Self {
        self.aad = aad.into();
        self
    }

    fn encrypt(&mut self, nonce: &[u8], data: &[u8]) -> Vec<u8> {
        let (mut output, tag) = ::ascon::aead_encrypt(&self.key, nonce, data, &self.aad);
        output.extend_from_slice(&tag);
        output
    }

    fn decrypt(&mut self, nonce: &[u8], data: &[u8]) -> Result<Vec<u8>, DecryptFail> {
        let (data, tag) = data.split_at(data.len() - Self::tag_length());
        ::ascon::aead_decrypt(&self.key, nonce, data, &self.aad, tag).map_err(|err| err.into())
    }
}

impl From<::ascon::DecryptFail> for DecryptFail {
    fn from(err: ::ascon::DecryptFail) -> DecryptFail {
        match err {
            ::ascon::DecryptFail::TagLengthError => DecryptFail::LengthError,
            ::ascon::DecryptFail::AuthenticationFail => DecryptFail::AuthenticationFail
        }
    }
}
