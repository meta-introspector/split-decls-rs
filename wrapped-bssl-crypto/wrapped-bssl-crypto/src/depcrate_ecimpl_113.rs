// Generated macro for impl_113 (impl)
macro_rules! Depcrate_ecimpl_113 {
() => {
// Module: crate::ec
// Provides: {"impl_113"}
// Dependencies: {}
impl Drop for Point { fn drop (& mut self) { unsafe { bssl_sys :: EC_POINT_free (self . point) } } }
};
}
