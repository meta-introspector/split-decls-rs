// Generated macro for impl_606 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_606 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_606"}
// Dependencies: {}
impl < 'str_ref > From < & 'str_ref JNIStr > for Cow < 'str_ref , str > { fn from (other : & 'str_ref JNIStr) -> Cow < 'str_ref , str > { let bytes = other . as_cstr () . to_bytes () ; match from_java_cesu8 (bytes) { Ok (s) => s , Err (e) => { debug ! ("error decoding java cesu8: {:#?}" , e) ; String :: from_utf8_lossy (bytes) } } } }
};
}
