// Generated macro for check_js_comment_close (function)
macro_rules! Depcrate_parsercheck_js_comment_close {
() => {
// Module: crate::parser
// Provides: {"check_js_comment_close"}
// Dependencies: {}
# [doc = " Return an [`Err`] if the given string contains a comment close syntax (`*/``)."] fn check_js_comment_close (str : & str , span : Span) -> Result < () , Diagnostic > { if str . contains ("*/") { Err (Diagnostic :: span_error (span , "contains comment close syntax" ,)) } else { Ok (()) } }
};
}
