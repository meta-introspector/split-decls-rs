// Generated macro for impl_28 (impl)
macro_rules! Depcrate_aeadimpl_28 {
() => {
// Module: crate::aead
// Provides: {"impl_28"}
// Dependencies: {}
impl < const KEY_LEN : usize , const NONCE_LEN : usize , const TAG_LEN : usize > Drop for EvpAead < KEY_LEN , NONCE_LEN , TAG_LEN > { fn drop (& mut self) { unsafe { bssl_sys :: EVP_AEAD_CTX_free (self . 0) } } }
};
}
