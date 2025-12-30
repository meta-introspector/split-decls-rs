// Generated macro for emit_suggestion (function)
macro_rules! Depcrate_suspicious_operation_groupingsemit_suggestion {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"emit_suggestion"}
// Dependencies: {}
fn emit_suggestion (cx : & EarlyContext < '_ > , span : Span , sugg : String , applicability : Applicability) { span_lint_and_sugg (cx , SUSPICIOUS_OPERATION_GROUPINGS , span , "this sequence of operators looks suspiciously like a bug" , "did you mean" , sugg , applicability ,) ; }
};
}
