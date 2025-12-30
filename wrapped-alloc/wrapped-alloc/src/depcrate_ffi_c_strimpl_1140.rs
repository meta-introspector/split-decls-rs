// Generated macro for impl_1140 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1140 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1140"}
// Dependencies: {}
impl NulError { # [doc = " Returns the position of the nul byte in the slice that caused"] # [doc = " [`CString::new`] to fail."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::ffi::CString;"] # [doc = ""] # [doc = " let nul_error = CString::new(\"foo\\0bar\").unwrap_err();"] # [doc = " assert_eq!(nul_error.nul_position(), 3);"] # [doc = ""] # [doc = " let nul_error = CString::new(\"foo bar\\0\").unwrap_err();"] # [doc = " assert_eq!(nul_error.nul_position(), 7);"] # [doc = " ```"] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] pub fn nul_position (& self) -> usize { self . 0 } # [doc = " Consumes this error, returning the underlying vector of bytes which"] # [doc = " generated the error in the first place."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::ffi::CString;"] # [doc = ""] # [doc = " let nul_error = CString::new(\"foo\\0bar\").unwrap_err();"] # [doc = " assert_eq!(nul_error.into_vec(), b\"foo\\0bar\");"] # [doc = " ```"] # [must_use = "`self` will be dropped if the result is not used"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn into_vec (self) -> Vec < u8 > { self . 1 } }
};
}
