// Generated macro for BytesOrWideString (enum)
macro_rules! Depcrate_typesBytesOrWideString {
() => {
// Module: crate::types
// Provides: {"BytesOrWideString"}
// Dependencies: {}
# [doc = " A platform independent representation of a string. When working with `std`"] # [doc = " enabled it is recommended to the convenience methods for providing"] # [doc = " conversions to `std` types."] # [derive (Debug)] pub enum BytesOrWideString < 'a > { # [doc = " A slice, typically provided on Unix platforms."] Bytes (& 'a [u8]) , # [doc = " Wide strings typically from Windows."] Wide (& 'a [u16]) , }
};
}
