// Generated macro for impl_79 (impl)
macro_rules! Depcrate_cipherimpl_79 {
() => {
// Module: crate::cipher
// Provides: {"impl_79"}
// Dependencies: {}
impl < C : EvpCipherType > Drop for Cipher < C > { fn drop (& mut self) { unsafe { bssl_sys :: EVP_CIPHER_CTX_free (self . ctx) } } }
};
}
