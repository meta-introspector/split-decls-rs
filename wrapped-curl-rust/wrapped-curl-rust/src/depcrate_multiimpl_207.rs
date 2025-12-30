// Generated macro for impl_207 (impl)
macro_rules! Depcrate_multiimpl_207 {
() => {
// Module: crate::multi
// Provides: {"impl_207"}
// Dependencies: {}
impl Drop for RawMulti { fn drop (& mut self) { unsafe { let _ = cvt (curl_sys :: curl_multi_cleanup (self . handle)) ; } } }
};
}
