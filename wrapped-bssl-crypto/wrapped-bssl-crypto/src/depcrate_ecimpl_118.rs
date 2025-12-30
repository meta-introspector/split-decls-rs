// Generated macro for impl_118 (impl)
macro_rules! Depcrate_ecimpl_118 {
() => {
// Module: crate::ec
// Provides: {"impl_118"}
// Dependencies: {}
impl Drop for Key { fn drop (& mut self) { unsafe { bssl_sys :: EC_KEY_free (self . 0) } } }
};
}
