// Generated macro for NulError (struct)
macro_rules! Depcrate_ffi_c_strNulError {
() => {
// Module: crate::ffi::c_str
// Provides: {"NulError"}
// Dependencies: {}
# [doc = " An error indicating that an interior nul byte was found."] # [doc = ""] # [doc = " While Rust strings may contain nul bytes in the middle, C strings"] # [doc = " can't, as that byte would effectively truncate the string."] # [doc = ""] # [doc = " This error is created by the [`new`][`CString::new`] method on"] # [doc = " [`CString`]. See its documentation for more."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::ffi::{CString, NulError};"] # [doc = ""] # [doc = " let _: NulError = CString::new(b\"f\\0oo\".to_vec()).unwrap_err();"] # [doc = " ```"] # [derive (Clone , PartialEq , Eq , Debug)] # [stable (feature = "alloc_c_string" , since = "1.64.0")] pub struct NulError (usize , Vec < u8 >) ;
};
}
