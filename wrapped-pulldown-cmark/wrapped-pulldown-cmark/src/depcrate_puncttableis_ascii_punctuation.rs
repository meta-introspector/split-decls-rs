// Generated macro for is_ascii_punctuation (function)
macro_rules! Depcrate_puncttableis_ascii_punctuation {
() => {
// Module: crate::puncttable
// Provides: {"is_ascii_punctuation"}
// Dependencies: {}
pub (crate) fn is_ascii_punctuation (c : u8) -> bool { c < 128 && (PUNCT_MASKS_ASCII [(c / 16) as usize] & (1 << (c & 15))) != 0 }
};
}
