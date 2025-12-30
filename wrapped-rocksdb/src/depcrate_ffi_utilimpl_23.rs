// Generated macro for impl_23 (impl)
macro_rules! Depcrate_ffi_utilimpl_23 {
() => {
// Module: crate::ffi_util
// Provides: {"impl_23"}
// Dependencies: {}
impl Drop for CSlice { fn drop (& mut self) { unsafe { ffi :: rocksdb_free (self . data as * mut c_void) ; } } }
};
}
