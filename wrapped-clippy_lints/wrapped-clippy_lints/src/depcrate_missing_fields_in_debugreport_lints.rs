// Generated macro for report_lints (function)
macro_rules! Depcrate_missing_fields_in_debugreport_lints {
() => {
// Module: crate::missing_fields_in_debug
// Provides: {"report_lints"}
// Dependencies: {}
fn report_lints (cx : & LateContext < '_ > , span : Span , span_notes : Vec < (Span , & 'static str) >) { span_lint_and_then (cx , MISSING_FIELDS_IN_DEBUG , span , "manual `Debug` impl does not include all fields" , | diag | { for (span , note) in span_notes { diag . span_note (span , note) ; } diag . help ("consider including all fields in this `Debug` impl") . help ("consider calling `.finish_non_exhaustive()` if you intend to ignore fields") ; } ,) ; }
};
}
