// Generated macro for impl_1119 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1119 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1119"}
// Dependencies: {}
# [stable (feature = "cstring_from_cow_cstr" , since = "1.28.0")] impl < 'a > From < Cow < 'a , CStr > > for CString { # [doc = " Converts a `Cow<'a, CStr>` into a `CString`, by copying the contents if they are"] # [doc = " borrowed."] # [inline] fn from (s : Cow < 'a , CStr >) -> Self { s . into_owned () } }
};
}
