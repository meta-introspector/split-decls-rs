// Generated macro for is_valid_scheme_char (function)
macro_rules! Depcrate_simple_urlis_valid_scheme_char {
() => {
// Module: crate::simple_url
// Provides: {"is_valid_scheme_char"}
// Dependencies: {}
# [doc = " Check if a character is valid in a URL scheme."] # [doc = " Valid scheme characters: alphanumeric, +, -, or ."] fn is_valid_scheme_char (c : char) -> bool { c . is_ascii_alphanumeric () || c == '+' || c == '-' || c == '.' }
};
}
