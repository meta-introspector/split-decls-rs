// Generated macro for get_eof_char (function)
macro_rules! Depcrateget_eof_char {
() => {
// Module: crate
// Provides: {"get_eof_char"}
// Dependencies: {}
fn get_eof_char () -> u8 { get_this_term_char (SpecialCharacterIndices :: VEOF) . unwrap_or (DEFAULT_VEOF_CHAR) }
};
}
