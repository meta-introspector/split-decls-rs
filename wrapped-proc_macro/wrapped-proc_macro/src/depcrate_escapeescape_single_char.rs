// Generated macro for escape_single_char (function)
macro_rules! Depcrate_escapeescape_single_char {
() => {
// Module: crate::escape
// Provides: {"escape_single_char"}
// Dependencies: {}
fn escape_single_char (ch : char , opt : EscapeOptions , repr : & mut String) { if (ch == '\'' && ! opt . escape_single_quote) || (ch == '"' && ! opt . escape_double_quote) { repr . push (ch) ; } else { repr . extend (ch . escape_debug ()) ; } }
};
}
