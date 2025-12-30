// Generated macro for get_intr_char (function)
macro_rules! Depcrateget_intr_char {
() => {
// Module: crate
// Provides: {"get_intr_char"}
// Dependencies: {}
fn get_intr_char () -> u8 { get_this_term_char (SpecialCharacterIndices :: VINTR) . unwrap_or (DEFAULT_INTR_CHAR) }
};
}
