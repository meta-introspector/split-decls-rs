// Generated macro for impl_223 (impl)
macro_rules! Depcrate_oid_arrayimpl_223 {
() => {
// Module: crate::oid_array
// Provides: {"impl_223"}
// Dependencies: {}
impl Drop for OidArray { fn drop (& mut self) { unsafe { raw :: git_oidarray_free (& mut self . raw) } } }
};
}
