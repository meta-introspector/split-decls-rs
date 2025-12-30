// Generated macro for impl_605 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_605 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_605"}
// Dependencies: {}
impl From < JNIString > for CString { fn from (string : JNIString) -> Self { string . into_cstring () } }
};
}
