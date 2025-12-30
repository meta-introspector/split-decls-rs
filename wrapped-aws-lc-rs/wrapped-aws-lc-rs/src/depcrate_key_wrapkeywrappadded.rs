// Generated macro for KeyWrapPadded (trait)
macro_rules! Depcrate_key_wrapKeyWrapPadded {
() => {
// Module: crate::key_wrap
// Provides: {"KeyWrapPadded"}
// Dependencies: {}
# [doc = " A Key Wrap with Padding (KWP) algorithm implementation."] # [allow (clippy :: module_name_repetitions)] pub trait KeyWrapPadded : Sealed { # [doc = " Peforms the key wrap padding encryption algorithm using a block cipher."] # [doc = " It wraps and pads `plaintext` writes the corresponding ciphertext to `output`."] # [doc = ""] # [doc = " # Errors"] # [doc = " * [`Unspecified`]: Any error that has occurred performing the operation."] fn wrap_with_padding < 'output > (self , plaintext : & [u8] , output : & 'output mut [u8] ,) -> Result < & 'output mut [u8] , Unspecified > ; # [doc = " Peforms the key wrap padding decryption algorithm using a block cipher."] # [doc = " It unwraps the padded `ciphertext` and writes the corresponding plaintext to `output`."] # [doc = ""] # [doc = " # Errors"] # [doc = " * [`Unspecified`]: Any error that has occurred performing the operation."] fn unwrap_with_padding < 'output > (self , ciphertext : & [u8] , output : & 'output mut [u8] ,) -> Result < & 'output mut [u8] , Unspecified > ; }
};
}
