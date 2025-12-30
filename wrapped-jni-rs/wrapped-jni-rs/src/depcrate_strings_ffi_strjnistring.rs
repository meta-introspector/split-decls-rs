// Generated macro for JNIString (struct)
macro_rules! Depcrate_strings_ffi_strJNIString {
() => {
// Module: crate::strings::ffi_str
// Provides: {"JNIString"}
// Dependencies: {}
# [doc = " An owned null-terminated string (like [`CString`]) encoded in Java's"] # [doc = " [modified UTF-8]."] # [doc = ""] # [doc = " This type is intended for constructing Java strings from Rust code. To use"] # [doc = " it, first construct an ordinary Rust [`str`] or [`String`], then use"] # [doc = " [`JNIString::new`] to convert it to the encoding that Java expects."] # [doc = ""] # [doc = " As with `CString`, this type has a borrowed counterpart, [`JNIStr`], that"] # [doc = " it coerces to."] # [doc = ""] # [doc = " [modified UTF-8]: https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8"] # [derive (Debug , PartialEq , PartialOrd , Eq , Ord , Hash , Clone)] pub struct JNIString { internal : CString , }
};
}
