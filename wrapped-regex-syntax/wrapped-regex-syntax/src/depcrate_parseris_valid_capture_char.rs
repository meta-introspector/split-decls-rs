// Generated macro for is_valid_capture_char (function)
macro_rules! Depcrate_parseris_valid_capture_char {
() => {
// Module: crate::parser
// Provides: {"is_valid_capture_char"}
// Dependencies: {}
fn is_valid_capture_char (c : char) -> bool { c == '_' || (c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') }
};
}
