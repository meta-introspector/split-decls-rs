// Generated macro for IntoStringError (struct)
macro_rules! Depcrate_ffi_c_strIntoStringError {
() => {
// Module: crate::ffi::c_str
// Provides: {"IntoStringError"}
// Dependencies: {}
# [doc = " An error indicating invalid UTF-8 when converting a [`CString`] into a [`String`]."] # [doc = ""] # [doc = " `CString` is just a wrapper over a buffer of bytes with a nul terminator;"] # [doc = " [`CString::into_string`] performs UTF-8 validation on those bytes and may"] # [doc = " return this error."] # [doc = ""] # [doc = " This `struct` is created by [`CString::into_string()`]. See"] # [doc = " its documentation for more."] # [derive (Clone , PartialEq , Eq , Debug)] # [stable (feature = "alloc_c_string" , since = "1.64.0")] pub struct IntoStringError { inner : CString , error : Utf8Error , }
};
}
