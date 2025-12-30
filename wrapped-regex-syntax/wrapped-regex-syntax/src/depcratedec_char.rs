// Generated macro for dec_char (function)
macro_rules! Depcratedec_char {
() => {
// Module: crate
// Provides: {"dec_char"}
// Dependencies: {}
fn dec_char (c : char) -> char { match c { '\x00' => '\x00' , '\u{E000}' => '\u{D7FF}' , c => char :: from_u32 (c as u32 - 1) . unwrap () , } }
};
}
