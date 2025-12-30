// Generated macro for is_char_boundary (function)
macro_rules! Depcrate_stris_char_boundary {
() => {
// Module: crate::str
// Provides: {"is_char_boundary"}
// Dependencies: {}
# [doc = " Test if a byte is the start of a UTF-8 character."] # [doc = " (extracted from `str::is_char_boundary`)"] # [inline] fn is_char_boundary (b : u8) -> bool { (b as i8) >= - 0x40 }
};
}
