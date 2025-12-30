// Generated macro for FromVecWithNulError (struct)
macro_rules! Depcrate_ffi_c_strFromVecWithNulError {
() => {
// Module: crate::ffi::c_str
// Provides: {"FromVecWithNulError"}
// Dependencies: {}
# [doc = " An error indicating that a nul byte was not in the expected position."] # [doc = ""] # [doc = " The vector used to create a [`CString`] must have one and only one nul byte,"] # [doc = " positioned at the end."] # [doc = ""] # [doc = " This error is created by the [`CString::from_vec_with_nul`] method."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::ffi::{CString, FromVecWithNulError};"] # [doc = ""] # [doc = " let _: FromVecWithNulError = CString::from_vec_with_nul(b\"f\\0oo\".to_vec()).unwrap_err();"] # [doc = " ```"] # [derive (Clone , PartialEq , Eq , Debug)] # [stable (feature = "alloc_c_string" , since = "1.64.0")] pub struct FromVecWithNulError { error_kind : FromBytesWithNulErrorKind , bytes : Vec < u8 > , }
};
}
