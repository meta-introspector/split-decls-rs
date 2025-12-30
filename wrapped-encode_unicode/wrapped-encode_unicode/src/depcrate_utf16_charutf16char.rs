// Generated macro for Utf16Char (struct)
macro_rules! Depcrate_utf16_charUtf16Char {
() => {
// Module: crate::utf16_char
// Provides: {"Utf16Char"}
// Dependencies: {}
# [derive (Default)] # [derive (PartialEq , Eq)] # [derive (Clone , Copy)] # [doc = " An unicode codepoint stored as UTF-16."] # [doc = ""] # [doc = " It can be borrowed as an `u16` slice, and has the same size as `char`."] pub struct Utf16Char { units : [u16 ; 2] , }
};
}
