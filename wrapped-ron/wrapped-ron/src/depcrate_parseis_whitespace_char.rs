// Generated macro for is_whitespace_char (function)
macro_rules! Depcrate_parseis_whitespace_char {
() => {
// Module: crate::parse
// Provides: {"is_whitespace_char"}
// Dependencies: {}
pub const fn is_whitespace_char (c : char) -> bool { matches ! (c , ' ' | '\t' | '\n' | '\r' | '\x0B' | '\x0C' | '\u{85}' | '\u{200E}' | '\u{200F}' | '\u{2028}' | '\u{2029}') }
};
}
