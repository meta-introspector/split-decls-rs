// Generated macro for inc_char (function)
macro_rules! Depcrateinc_char {
() => {
// Module: crate
// Provides: {"inc_char"}
// Dependencies: {}
fn inc_char (c : char) -> char { match c { char :: MAX => char :: MAX , '\u{D7FF}' => '\u{E000}' , c => char :: from_u32 (c as u32 + 1) . unwrap () , } }
};
}
