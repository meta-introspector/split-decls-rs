// Generated macro for encrypt (function)
macro_rules! Depcrate_symmencrypt {
() => {
// Module: crate::symm
// Provides: {"encrypt"}
// Dependencies: {}
# [doc = " Encrypts data in one go, and returns the encrypted data."] # [doc = ""] # [doc = " Data is encrypted using the specified cipher type `t` in encrypt mode with the specified `key`"] # [doc = " and initialization vector `iv`. Padding is enabled."] # [doc = ""] # [doc = " This is a convenient interface to `Crypter` to encrypt all data in one go.  To encrypt a stream"] # [doc = " of data incrementally , use `Crypter` instead."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Encrypt data in AES128 CBC mode"] # [doc = ""] # [doc = " ```"] # [doc = " use openssl::symm::{encrypt, Cipher};"] # [doc = ""] # [doc = " let cipher = Cipher::aes_128_cbc();"] # [doc = " let data = b\"Some Crypto Text\";"] # [doc = " let key = b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\x09\\x0A\\x0B\\x0C\\x0D\\x0E\\x0F\";"] # [doc = " let iv = b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\";"] # [doc = " let ciphertext = encrypt("] # [doc = "     cipher,"] # [doc = "     key,"] # [doc = "     Some(iv),"] # [doc = "     data).unwrap();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     b\"\\xB4\\xB9\\xE7\\x30\\xD6\\xD6\\xF7\\xDE\\x77\\x3F\\x1C\\xFF\\xB3\\x3E\\x44\\x5A\\x91\\xD7\\x27\\x62\\x87\\x4D\\"] # [doc = "       \\xFB\\x3C\\x5E\\xC4\\x59\\x72\\x4A\\xF4\\x7C\\xA1\","] # [doc = "     &ciphertext[..]);"] # [doc = " ```"] pub fn encrypt (t : Cipher , key : & [u8] , iv : Option < & [u8] > , data : & [u8] ,) -> Result < Vec < u8 > , ErrorStack > { cipher (t , Mode :: Encrypt , key , iv , data) }
};
}
