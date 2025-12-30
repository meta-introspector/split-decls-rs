// Generated macro for impl_604 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_604 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_604"}
// Dependencies: {}
impl < T > From < T > for JNIString where T : AsRef < str > , { fn from (other : T) -> Self { let enc = to_java_cesu8 (other . as_ref ()) . into_owned () ; JNIString { internal : unsafe { CString :: from_vec_unchecked (enc) } , } } }
};
}
