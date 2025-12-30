// Generated macro for wrap_key (function)
macro_rules! Depcrate_aeswrap_key {
() => {
// Module: crate::aes
// Provides: {"wrap_key"}
// Dependencies: {}
# [doc = " Wrap a key, according to [RFC 3394](https://tools.ietf.org/html/rfc3394)"] # [doc = ""] # [doc = " * `key`: The key-encrypting-key to use. Must be a encrypting key"] # [doc = " * `iv`: The IV to use. You must use the same IV for both wrapping and unwrapping"] # [doc = " * `out`: The output buffer to store the ciphertext"] # [doc = " * `in_`: The input buffer, storing the key to be wrapped"] # [doc = ""] # [doc = " Returns the number of bytes written into `out`"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if either `out` or `in_` do not have sizes that are a multiple of 8, or if"] # [doc = " `out` is not 8 bytes longer than `in_`"] # [corresponds (AES_wrap_key)] pub fn wrap_key (key : & AesKey , iv : Option < [u8 ; 8] > , out : & mut [u8] , in_ : & [u8] ,) -> Result < usize , KeyError > { unsafe { assert ! (out . len () >= in_ . len () + 8) ; let written = ffi :: AES_wrap_key (& key . 0 as * const _ as * mut _ , iv . as_ref () . map_or (ptr :: null () , | iv | iv . as_ptr () as * const _) , out . as_ptr () as * mut _ , in_ . as_ptr () as * const _ , in_ . len () as AesSizeType ,) ; if written <= 0 { Err (KeyError (())) } else { Ok (written as usize) } } }
};
}
