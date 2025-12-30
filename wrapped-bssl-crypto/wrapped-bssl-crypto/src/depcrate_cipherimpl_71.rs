// Generated macro for impl_71 (impl)
macro_rules! Depcrate_cipherimpl_71 {
() => {
// Module: crate::cipher
// Provides: {"impl_71"}
// Dependencies: {}
impl EvpCipherType for EvpAes256Ctr { type Key = [u8 ; 32] ; type Nonce = [u8 ; 16] ; fn evp_cipher () -> * const EVP_CIPHER { unsafe { bssl_sys :: EVP_aes_256_ctr () } } }
};
}
