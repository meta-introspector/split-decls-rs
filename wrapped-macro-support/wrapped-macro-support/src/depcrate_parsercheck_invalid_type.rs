// Generated macro for check_invalid_type (function)
macro_rules! Depcrate_parsercheck_invalid_type {
() => {
// Module: crate::parser
// Provides: {"check_invalid_type"}
// Dependencies: {}
# [doc = " Return an [`Err`] if the given string is a JS keyword or contains a comment close syntax (`*/``)."] fn check_invalid_type (str : & str , span : Span) -> Result < () , Diagnostic > { if is_js_keyword (str) { return Err (Diagnostic :: span_error (span , "collides with JS keyword")) ; } check_js_comment_close (str , span) ? ; Ok (()) }
};
}
