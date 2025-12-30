// Generated macro for parse_prefix (function)
macro_rules! Depcrate_windows_term_colorsparse_prefix {
() => {
// Module: crate::windows_term::colors
// Provides: {"parse_prefix"}
// Dependencies: {}
fn parse_prefix (bytes : & mut Bytes < '_ >) -> Option < () > { if bytes . next () ? == b'[' { Some (()) } else { None } }
};
}
