// Generated macro for EvpAead (struct)
macro_rules! Depcrate_aeadEvpAead {
() => {
// Module: crate::aead
// Provides: {"EvpAead"}
// Dependencies: {}
# [doc = " An internal struct that implements AEAD operations given an `EVP_AEAD`."] struct EvpAead < const KEY_LEN : usize , const NONCE_LEN : usize , const TAG_LEN : usize > (* mut bssl_sys :: EVP_AEAD_CTX ,) ;
};
}
