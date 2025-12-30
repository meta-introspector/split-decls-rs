// Generated macro for lex_error (function)
macro_rules! Depcrate_parselex_error {
() => {
// Module: crate::parse
// Provides: {"lex_error"}
// Dependencies: {}
fn lex_error (cursor : Cursor) -> LexError { # [cfg (not (span_locations))] let _ = cursor ; LexError { span : Span { # [cfg (span_locations)] lo : cursor . off , # [cfg (span_locations)] hi : cursor . off , } , } }
};
}
