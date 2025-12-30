// Generated macro for is_section_char (function)
macro_rules! Depcrate_parse_nomis_section_char {
() => {
// Module: crate::parse::nom
// Provides: {"is_section_char"}
// Dependencies: {}
fn is_section_char (c : u8) -> bool { c . is_ascii_alphanumeric () || c == b'-' || c == b'.' }
};
}
