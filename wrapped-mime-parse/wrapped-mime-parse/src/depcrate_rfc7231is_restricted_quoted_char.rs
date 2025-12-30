// Generated macro for is_restricted_quoted_char (function)
macro_rules! Depcrate_rfc7231is_restricted_quoted_char {
() => {
// Module: crate::rfc7231
// Provides: {"is_restricted_quoted_char"}
// Dependencies: {}
fn is_restricted_quoted_char (c : u8) -> bool { c == 9 || (c > 31 && c != 127) }
};
}
