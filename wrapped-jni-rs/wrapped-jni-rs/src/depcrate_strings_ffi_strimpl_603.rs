// Generated macro for impl_603 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_603 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_603"}
// Dependencies: {}
impl PartialEq < JNIString > for & JNIStr { # [inline] fn eq (& self , other : & JNIString) -> bool { & self . internal == other . internal . as_c_str () } }
};
}
