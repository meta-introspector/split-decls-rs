// Generated macro for Utf8Char (struct)
macro_rules! Depcrate_utf8_charUtf8Char {
() => {
// Module: crate::utf8_char
// Provides: {"Utf8Char"}
// Dependencies: {}
# [derive (Default)] # [derive (PartialEq , Eq , PartialOrd , Ord)] # [derive (Clone , Copy)] # [doc = " An unicode codepoint stored as UTF-8."] # [doc = ""] # [doc = " It can be borrowed as a `str`, and has the same size as `char`."] pub struct Utf8Char { bytes : [u8 ; 4] , }
};
}
