// Generated macro for impl_301 (impl)
macro_rules! Depcrate_scopedimpl_301 {
() => {
// Module: crate::scoped
// Provides: {"impl_301"}
// Dependencies: {}
impl Drop for Bignum { fn drop (& mut self) { unsafe { bssl_sys :: BN_free (& mut self . 0) } } }
};
}
