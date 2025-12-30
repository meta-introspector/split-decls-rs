// Generated macro for is_punctuation (function)
macro_rules! Depcrate_puncttableis_punctuation {
() => {
// Module: crate::puncttable
// Provides: {"is_punctuation"}
// Dependencies: {}
pub (crate) fn is_punctuation (c : char) -> bool { let cp = c as u32 ; if cp < 128 { return is_ascii_punctuation (cp as u8) ; } if cp > 0x1FBCA { return false ; } let high = (cp / 16) as u16 ; match PUNCT_TAB . binary_search (& high) { Ok (index) => (PUNCT_MASKS [index] & (1 << (cp & 15))) != 0 , _ => false , } }
};
}
