// Generated macro for make_span_lint_and_sugg (function)
macro_rules! Depcrate_manual_retainmake_span_lint_and_sugg {
() => {
// Module: crate::manual_retain
// Provides: {"make_span_lint_and_sugg"}
// Dependencies: {}
fn make_span_lint_and_sugg (cx : & LateContext < '_ > , span : Span , sugg : String) { span_lint_and_sugg (cx , MANUAL_RETAIN , span , "this expression can be written more simply using `.retain()`" , "consider calling `.retain()` instead" , sugg , Applicability :: MachineApplicable ,) ; }
};
}
