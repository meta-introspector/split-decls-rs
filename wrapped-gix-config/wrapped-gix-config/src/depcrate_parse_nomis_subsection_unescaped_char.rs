// Generated macro for is_subsection_unescaped_char (function)
macro_rules! Depcrate_parse_nomis_subsection_unescaped_char {
() => {
// Module: crate::parse::nom
// Provides: {"is_subsection_unescaped_char"}
// Dependencies: {}
fn is_subsection_unescaped_char (c : u8) -> bool { c != b'"' && c != b'\\' && c != b'\n' && c != 0 }
};
}
