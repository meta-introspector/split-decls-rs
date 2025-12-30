// Generated macro for is_span_if (function)
macro_rules! Depcrate_check_proc_macrois_span_if {
() => {
// Module: crate::check_proc_macro
// Provides: {"is_span_if"}
// Dependencies: {}
# [doc = " Checks if the span actually refers to an if expression"] pub fn is_span_if (cx : & impl LintContext , span : Span) -> bool { span_matches_pat (cx . sess () , span , Pat :: Str ("if") , Pat :: Str ("}")) }
};
}
