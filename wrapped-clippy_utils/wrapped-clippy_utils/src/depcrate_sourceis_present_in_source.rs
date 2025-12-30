// Generated macro for is_present_in_source (function)
macro_rules! Depcrate_sourceis_present_in_source {
() => {
// Module: crate::source
// Provides: {"is_present_in_source"}
// Dependencies: {}
pub fn is_present_in_source (sess : & impl HasSession , span : Span) -> bool { if let Some (snippet) = snippet_opt (sess , span) && snippet . is_empty () { return false ; } true }
};
}
