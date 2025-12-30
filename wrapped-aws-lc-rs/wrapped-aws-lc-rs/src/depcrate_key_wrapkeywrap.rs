// Generated macro for KeyWrap (trait)
macro_rules! Depcrate_key_wrapKeyWrap {
() => {
// Module: crate::key_wrap
// Provides: {"KeyWrap"}
// Dependencies: {}
# [doc = " A Key Wrap (KW) algorithm implementation."] # [allow (clippy :: module_name_repetitions)] pub trait KeyWrap : Sealed { # [doc = " Peforms the key wrap encryption algorithm using a block cipher."] # [doc = " It wraps `plaintext` and writes the corresponding ciphertext to `output`."] # [doc = ""] # [doc = " # Errors"] # [doc = " * [`Unspecified`]: Any error that has occurred performing the operation."] fn wrap < 'output > (self , plaintext : & [u8] , output : & 'output mut [u8] ,) -> Result < & 'output mut [u8] , Unspecified > ; # [doc = " Peforms the key wrap decryption algorithm using a block cipher."] # [doc = " It unwraps `ciphertext` and writes the corresponding plaintext to `output`."] # [doc = ""] # [doc = " # Errors"] # [doc = " * [`Unspecified`]: Any error that has occurred performing the operation."] fn unwrap < 'output > (self , ciphertext : & [u8] , output : & 'output mut [u8] ,) -> Result < & 'output mut [u8] , Unspecified > ; }
};
}
