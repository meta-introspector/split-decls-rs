// Generated macro for unwrap_key (function)
macro_rules! Depcrate_aesunwrap_key {
() => {
// Module: crate::aes
// Provides: {"unwrap_key"}
// Dependencies: {}
# [doc = " Unwrap a key, according to [RFC 3394](https://tools.ietf.org/html/rfc3394)"] # [doc = ""] # [doc = " * `key`: The key-encrypting-key to decrypt the wrapped key. Must be a decrypting key"] # [doc = " * `iv`: The same IV used for wrapping the key"] # [doc = " * `out`: The buffer to write the unwrapped key to"] # [doc = " * `in_`: The input ciphertext"] # [doc = ""] # [doc = " Returns the number of bytes written into `out`"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if either `out` or `in_` do not have sizes that are a multiple of 8, or"] # [doc = " if `in_` is not 8 bytes longer than `out`"] # [corresponds (AES_unwrap_key)] pub fn unwrap_key (key : & AesKey , iv : Option < [u8 ; 8] > , out : & mut [u8] , in_ : & [u8] ,) -> Result < usize , KeyError > { unsafe { assert ! (out . len () + 8 <= in_ . len ()) ; let written = ffi :: AES_unwrap_key (& key . 0 as * const _ as * mut _ , iv . as_ref () . map_or (ptr :: null () , | iv | iv . as_ptr () as * const _) , out . as_ptr () as * mut _ , in_ . as_ptr () as * const _ , in_ . len () as AesSizeType ,) ; if written <= 0 { Err (KeyError (())) } else { Ok (written as usize) } } }
};
}
