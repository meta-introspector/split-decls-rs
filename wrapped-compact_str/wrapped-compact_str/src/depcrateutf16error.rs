// Generated macro for Utf16Error (struct)
macro_rules! DepcrateUtf16Error {
() => {
// Module: crate
// Provides: {"Utf16Error"}
// Dependencies: {}
# [doc = " A possible error value when converting a [`CompactString`] from a UTF-16 byte slice."] # [doc = ""] # [doc = " This type is the error type for the [`from_utf16`] method on [`CompactString`]."] # [doc = ""] # [doc = " [`from_utf16`]: CompactString::from_utf16"] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " # use compact_str::CompactString;"] # [doc = " // 𝄞mu<invalid>ic"] # [doc = " let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,"] # [doc = "           0xD800, 0x0069, 0x0063];"] # [doc = ""] # [doc = " assert!(CompactString::from_utf16(v).is_err());"] # [doc = " ```"] # [derive (Copy , Clone , Debug)] pub struct Utf16Error (()) ;
};
}
