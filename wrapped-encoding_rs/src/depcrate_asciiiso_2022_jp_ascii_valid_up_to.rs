// Generated macro for iso_2022_jp_ascii_valid_up_to (function)
macro_rules! Depcrate_asciiiso_2022_jp_ascii_valid_up_to {
() => {
// Module: crate::ascii
// Provides: {"iso_2022_jp_ascii_valid_up_to"}
// Dependencies: {}
pub fn iso_2022_jp_ascii_valid_up_to (bytes : & [u8]) -> usize { for (i , b_ref) in bytes . iter () . enumerate () { let b = * b_ref ; if b >= 0x80 || b == 0x1B || b == 0x0E || b == 0x0F { return i ; } } bytes . len () }
};
}
