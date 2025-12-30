// Generated macro for get_this_term_char (function)
macro_rules! Depcrateget_this_term_char {
() => {
// Module: crate
// Provides: {"get_this_term_char"}
// Dependencies: {}
fn get_this_term_char (char : SpecialCharacterIndices) -> Option < u8 > { for & fd in & [STDIN_FILENO , STDOUT_FILENO] { if let Ok (char) = get_term_char (fd , char) { return Some (char) ; } } None }
};
}
