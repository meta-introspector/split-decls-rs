// Generated macro for impl_291 (impl)
macro_rules! Depcrate_scopedimpl_291 {
() => {
// Module: crate::scoped
// Provides: {"impl_291"}
// Dependencies: {}
impl Drop for EcKey { fn drop (& mut self) { unsafe { bssl_sys :: EC_KEY_free (self . 0) } } }
};
}
