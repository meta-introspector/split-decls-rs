// Generated macro for get_data (function)
macro_rules! Depcrate_greek_to_meget_data {
() => {
// Module: crate::greek_to_me
// Provides: {"get_data"}
// Dependencies: {}
pub (crate) fn get_data (ch : char) -> Option < GreekPrecomposedLetterData > { let ch_i = ch as usize ; let packed = if (0x370 ..= 0x3FF) . contains (& ch_i) { * data :: DATA_370 . get (ch_i - 0x370) ? } else if (0x1f00 .. 0x1fff) . contains (& ch_i) { * data :: DATA_1F00 . get (ch_i - 0x1f00) ? } else { data :: match_extras (ch) ? } ; let packed = PackedGreekPrecomposedLetterData (packed) ; GreekPrecomposedLetterData :: try_from (packed) . ok () }
};
}
