// Generated macro for HmacKey (trait)
macro_rules! Depcrate_cryptoHmacKey {
() => {
// Module: crate::crypto
// Provides: {"HmacKey"}
// Dependencies: {}
# [doc = " A key for signing with HMAC-based algorithms"] pub trait HmacKey : Send + Sync { # [doc = " Method for signing a message"] fn sign (& self , data : & [u8] , signature_out : & mut [u8]) ; # [doc = " Length of `sign`'s output"] fn signature_len (& self) -> usize ; # [doc = " Method for verifying a message"] fn verify (& self , data : & [u8] , signature : & [u8]) -> Result < () , CryptoError > ; }
};
}
