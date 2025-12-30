// Generated macro for span_useless_format_empty (function)
macro_rules! Depcrate_formatspan_useless_format_empty {
() => {
// Module: crate::format
// Provides: {"span_useless_format_empty"}
// Dependencies: {}
fn span_useless_format_empty (cx : & LateContext < '_ > , span : Span , sugg : String , applicability : Applicability) { span_lint_and_sugg (cx , USELESS_FORMAT , span , "useless use of `format!`" , "consider using `String::new()`" , sugg , applicability ,) ; }
};
}
