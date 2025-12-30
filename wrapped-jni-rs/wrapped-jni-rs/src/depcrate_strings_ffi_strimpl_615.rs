// Generated macro for impl_615 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_615 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_615"}
// Dependencies: {}
impl ToOwned for JNIStr { type Owned = JNIString ; fn to_owned (& self) -> JNIString { JNIString { internal : CString :: from (self . as_cstr ()) , } } }
};
}
