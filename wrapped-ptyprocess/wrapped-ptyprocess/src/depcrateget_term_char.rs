// Generated macro for get_term_char (function)
macro_rules! Depcrateget_term_char {
() => {
// Module: crate
// Provides: {"get_term_char"}
// Dependencies: {}
fn get_term_char (fd : RawFd , char : SpecialCharacterIndices) -> Result < u8 > { let flags = termios :: tcgetattr (fd) ? ; let b = flags . control_chars [char as usize] ; Ok (b) }
};
}
