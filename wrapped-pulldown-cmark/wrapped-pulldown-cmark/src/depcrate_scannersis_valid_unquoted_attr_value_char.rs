// Generated macro for is_valid_unquoted_attr_value_char (function)
macro_rules! Depcrate_scannersis_valid_unquoted_attr_value_char {
() => {
// Module: crate::scanners
// Provides: {"is_valid_unquoted_attr_value_char"}
// Dependencies: {}
fn is_valid_unquoted_attr_value_char (c : u8) -> bool { ! matches ! (c , b'\'' | b'"' | b' ' | b'=' | b'>' | b'<' | b'`' | b'\n' | b'\r') }
};
}
