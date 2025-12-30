// Generated macro for impl_607 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_607 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_607"}
// Dependencies: {}
impl < 'str_ref > From < & 'str_ref JNIStr > for & 'str_ref CStr { fn from (value : & 'str_ref JNIStr) -> Self { & value . internal } }
};
}
