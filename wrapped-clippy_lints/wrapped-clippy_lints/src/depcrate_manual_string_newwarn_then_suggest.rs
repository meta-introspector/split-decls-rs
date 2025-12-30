// Generated macro for warn_then_suggest (function)
macro_rules! Depcrate_manual_string_newwarn_then_suggest {
() => {
// Module: crate::manual_string_new
// Provides: {"warn_then_suggest"}
// Dependencies: {}
fn warn_then_suggest (cx : & LateContext < '_ > , span : Span) { span_lint_and_sugg (cx , MANUAL_STRING_NEW , span , "empty String is being created manually" , "consider using" , "String::new()" . into () , MachineApplicable ,) ; }
};
}
