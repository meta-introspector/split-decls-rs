// Generated macro for utf8_char_width (function)
macro_rules! Depcrate_unicodeutf8_char_width {
() => {
// Module: crate::unicode
// Provides: {"utf8_char_width"}
// Dependencies: {}
# [doc = " Given a first byte, determine how many bytes are in this UTF-8 character"] # [inline] pub fn utf8_char_width (b : u8) -> usize { return UTF8_CHAR_WIDTH [b as usize] as usize ; }
};
}
