// Generated macro for impl_75 (impl)
macro_rules! Depcrate_cipherimpl_75 {
() => {
// Module: crate::cipher
// Provides: {"impl_75"}
// Dependencies: {}
impl EvpCipherType for EvpAes256Cbc { type Key = [u8 ; 32] ; type Nonce = [u8 ; 16] ; fn evp_cipher () -> * const EVP_CIPHER { unsafe { bssl_sys :: EVP_aes_256_cbc () } } }
};
}
