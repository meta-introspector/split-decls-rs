// Generated macro for first_char_in_first_line (function)
macro_rules! Depcrate_sourcefirst_char_in_first_line {
() => {
// Module: crate::source
// Provides: {"first_char_in_first_line"}
// Dependencies: {}
fn first_char_in_first_line (sess : & impl HasSession , span : Span) -> Option < BytePos > { let line_span = line_span (sess , span) ; snippet_opt (sess , line_span) . and_then (| snip | { snip . find (| c : char | ! c . is_whitespace ()) . map (| pos | line_span . lo () + BytePos :: from_usize (pos)) }) }
};
}
