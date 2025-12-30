// Generated macro for EvpCipherType (trait)
macro_rules! Depcrate_cipherEvpCipherType {
() => {
// Module: crate::cipher
// Provides: {"EvpCipherType"}
// Dependencies: {}
# [doc = " A cipher type, where `Key` is the size of the Key and `Nonce` is the size of the nonce or IV."] # [doc = " This must only be exposed publicly by types who ensure that `Key` is the correct size for the"] # [doc = " given CipherType. This can be checked via `bssl_sys::EVP_CIPHER_key_length`."] trait EvpCipherType { type Key : AsRef < [u8] > ; type Nonce : AsRef < [u8] > ; fn evp_cipher () -> * const EVP_CIPHER ; }
};
}
