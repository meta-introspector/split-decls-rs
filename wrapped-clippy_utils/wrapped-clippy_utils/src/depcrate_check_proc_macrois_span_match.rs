// Generated macro for is_span_match (function)
macro_rules! Depcrate_check_proc_macrois_span_match {
() => {
// Module: crate::check_proc_macro
// Provides: {"is_span_match"}
// Dependencies: {}
# [doc = " Checks if the span actually refers to a match expression"] pub fn is_span_match (cx : & impl LintContext , span : Span) -> bool { span_matches_pat (cx . sess () , span , Pat :: Str ("match") , Pat :: Str ("}")) }
};
}
