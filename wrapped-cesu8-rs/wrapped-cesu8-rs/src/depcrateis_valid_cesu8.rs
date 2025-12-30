// Generated macro for is_valid_cesu8 (function)
macro_rules! Depcrateis_valid_cesu8 {
() => {
// Module: crate
// Provides: {"is_valid_cesu8"}
// Dependencies: {}
# [doc = " Check whether a Rust string contains valid CESU-8 data."] pub fn is_valid_cesu8 (text : & str) -> bool { for b in text . bytes () { if (b & ! CONT_MASK) == TAG_CONT_U8 { continue ; } if utf8_char_width (b) > 3 { return false ; } } true }
};
}
