// Generated macro for impl_69 (impl)
macro_rules! Depcrate_cipherimpl_69 {
() => {
// Module: crate::cipher
// Provides: {"impl_69"}
// Dependencies: {}
impl EvpCipherType for EvpAes128Ctr { type Key = [u8 ; 16] ; type Nonce = [u8 ; 16] ; fn evp_cipher () -> * const EVP_CIPHER { unsafe { bssl_sys :: EVP_aes_128_ctr () } } }
};
}
