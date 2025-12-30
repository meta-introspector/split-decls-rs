// Generated macro for FromUtf16Error (struct)
macro_rules! Depcrate_stringFromUtf16Error {
() => {
// Module: crate::string
// Provides: {"FromUtf16Error"}
// Dependencies: {}
# [doc = " A possible error value when converting a `String` from a UTF-16 byte slice."] # [doc = ""] # [doc = " This type is the error type for the [`from_utf16`] method on [`String`]."] # [doc = ""] # [doc = " [`from_utf16`]: String::from_utf16"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " // 𝄞mu<invalid>ic"] # [doc = " let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,"] # [doc = "           0xD800, 0x0069, 0x0063];"] # [doc = ""] # [doc = " assert!(String::from_utf16(v).is_err());"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Debug)] pub struct FromUtf16Error (()) ;
};
}
