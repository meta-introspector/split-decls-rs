// Generated macro for impl_1143 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1143 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1143"}
// Dependencies: {}
impl IntoStringError { # [doc = " Consumes this error, returning original [`CString`] which generated the"] # [doc = " error."] # [must_use = "`self` will be dropped if the result is not used"] # [stable (feature = "cstring_into" , since = "1.7.0")] pub fn into_cstring (self) -> CString { self . inner } # [doc = " Access the underlying UTF-8 error that was the cause of this error."] # [must_use] # [stable (feature = "cstring_into" , since = "1.7.0")] pub fn utf8_error (& self) -> Utf8Error { self . error } }
};
}
