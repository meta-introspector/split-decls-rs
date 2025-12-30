// Generated macro for span_useless_format (function)
macro_rules! Depcrate_formatspan_useless_format {
() => {
// Module: crate::format
// Provides: {"span_useless_format"}
// Dependencies: {}
fn span_useless_format (cx : & LateContext < '_ > , span : Span , sugg : String , applicability : Applicability) { span_lint_and_sugg (cx , USELESS_FORMAT , span , "useless use of `format!`" , "consider using `.to_string()`" , sugg , applicability ,) ; }
};
}
