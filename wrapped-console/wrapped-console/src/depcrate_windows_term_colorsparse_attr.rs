// Generated macro for parse_attr (function)
macro_rules! Depcrate_windows_term_colorsparse_attr {
() => {
// Module: crate::windows_term::colors
// Provides: {"parse_attr"}
// Dependencies: {}
fn parse_attr (mut bytes : Bytes < '_ >) -> Option < u8 > { parse_prefix (& mut bytes) ? ; let attr = match bytes . next () ? { attr @ b'1' ..= b'8' => attr , _ => return None , } ; parse_suffix (& mut bytes) ? ; Some (attr) }
};
}
