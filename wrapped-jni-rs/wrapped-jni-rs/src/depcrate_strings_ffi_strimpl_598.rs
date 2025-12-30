// Generated macro for impl_598 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_598 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_598"}
// Dependencies: {}
impl PartialEq < & JNIStr > for JNIString { # [inline] fn eq (& self , other : & & JNIStr) -> bool { self . internal . as_c_str () == & other . internal } }
};
}
