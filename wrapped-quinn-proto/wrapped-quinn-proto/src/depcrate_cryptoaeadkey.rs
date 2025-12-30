// Generated macro for AeadKey (trait)
macro_rules! Depcrate_cryptoAeadKey {
() => {
// Module: crate::crypto
// Provides: {"AeadKey"}
// Dependencies: {}
# [doc = " A key for sealing data with AEAD-based algorithms"] pub trait AeadKey { # [doc = " Method for sealing message `data`"] fn seal (& self , data : & mut Vec < u8 > , additional_data : & [u8]) -> Result < () , CryptoError > ; # [doc = " Method for opening a sealed message `data`"] fn open < 'a > (& self , data : & 'a mut [u8] , additional_data : & [u8] ,) -> Result < & 'a mut [u8] , CryptoError > ; }
};
}
