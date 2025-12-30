// Generated macro for impl_73 (impl)
macro_rules! Depcrate_cipherimpl_73 {
() => {
// Module: crate::cipher
// Provides: {"impl_73"}
// Dependencies: {}
impl EvpCipherType for EvpAes128Cbc { type Key = [u8 ; 16] ; type Nonce = [u8 ; 16] ; fn evp_cipher () -> * const EVP_CIPHER { unsafe { bssl_sys :: EVP_aes_128_cbc () } } }
};
}
