// Generated macro for decrypt (function)
macro_rules! Depcrate_symmdecrypt {
() => {
// Module: crate::symm
// Provides: {"decrypt"}
// Dependencies: {}
# [doc = " Decrypts data in one go, and returns the decrypted data."] # [doc = ""] # [doc = " Data is decrypted using the specified cipher type `t` in decrypt mode with the specified `key`"] # [doc = " and initialization vector `iv`. Padding is enabled."] # [doc = ""] # [doc = " This is a convenient interface to `Crypter` to decrypt all data in one go.  To decrypt a  stream"] # [doc = " of data incrementally , use `Crypter` instead."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Decrypt data in AES128 CBC mode"] # [doc = ""] # [doc = " ```"] # [doc = " use openssl::symm::{decrypt, Cipher};"] # [doc = ""] # [doc = " let cipher = Cipher::aes_128_cbc();"] # [doc = " let data = b\"\\xB4\\xB9\\xE7\\x30\\xD6\\xD6\\xF7\\xDE\\x77\\x3F\\x1C\\xFF\\xB3\\x3E\\x44\\x5A\\x91\\xD7\\x27\\x62\\"] # [doc = "              \\x87\\x4D\\xFB\\x3C\\x5E\\xC4\\x59\\x72\\x4A\\xF4\\x7C\\xA1\";"] # [doc = " let key = b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\x09\\x0A\\x0B\\x0C\\x0D\\x0E\\x0F\";"] # [doc = " let iv = b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\";"] # [doc = " let ciphertext = decrypt("] # [doc = "     cipher,"] # [doc = "     key,"] # [doc = "     Some(iv),"] # [doc = "     data).unwrap();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     b\"Some Crypto Text\","] # [doc = "     &ciphertext[..]);"] # [doc = " ```"] pub fn decrypt (t : Cipher , key : & [u8] , iv : Option < & [u8] > , data : & [u8] ,) -> Result < Vec < u8 > , ErrorStack > { cipher (t , Mode :: Decrypt , key , iv , data) }
};
}
