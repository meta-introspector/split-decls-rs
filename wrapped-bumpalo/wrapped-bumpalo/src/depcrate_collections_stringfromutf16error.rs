// Generated macro for FromUtf16Error (struct)
macro_rules! Depcrate_collections_stringFromUtf16Error {
() => {
// Module: crate::collections::string
// Provides: {"FromUtf16Error"}
// Dependencies: {}
# [doc = " A possible error value when converting a `String` from a UTF-16 byte slice."] # [doc = ""] # [doc = " This type is the error type for the [`from_utf16_in`] method on [`String`]."] # [doc = ""] # [doc = " [`from_utf16_in`]: struct.String.html#method.from_utf16_in"] # [doc = " [`String`]: struct.String.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bumpalo::{Bump, collections::String};"] # [doc = ""] # [doc = " let b = Bump::new();"] # [doc = ""] # [doc = " // 𝄞mu<invalid>ic"] # [doc = " let v = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];"] # [doc = ""] # [doc = " assert!(String::from_utf16_in(v, &b).is_err());"] # [doc = " ```"] # [derive (Debug)] pub struct FromUtf16Error (()) ;
};
}
