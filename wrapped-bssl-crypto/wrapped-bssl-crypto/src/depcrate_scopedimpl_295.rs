// Generated macro for impl_295 (impl)
macro_rules! Depcrate_scopedimpl_295 {
() => {
// Module: crate::scoped
// Provides: {"impl_295"}
// Dependencies: {}
impl Drop for EvpHpkeCtx { fn drop (& mut self) { unsafe { bssl_sys :: EVP_HPKE_CTX_free (self . 0) } } }
};
}
