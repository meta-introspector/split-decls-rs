// Generated macro for impl_298 (impl)
macro_rules! Depcrate_scopedimpl_298 {
() => {
// Module: crate::scoped
// Provides: {"impl_298"}
// Dependencies: {}
impl Drop for EvpHpkeKey { fn drop (& mut self) { unsafe { bssl_sys :: EVP_HPKE_KEY_cleanup (& mut self . 0) } } }
};
}
