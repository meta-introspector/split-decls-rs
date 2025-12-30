// Generated macro for impl_24 (impl)
macro_rules! Depcrate_diagnosticimpl_24 {
() => {
// Module: crate::diagnostic
// Provides: {"impl_24"}
// Dependencies: {}
impl DiagnosticExt for Diagnostic { fn spanned_range (span_range : SpanRange , level : Level , message : String) -> Self { Diagnostic { level , span_range , msg : message , suggestions : vec ! [] , children : vec ! [] , } } fn span_range_error (mut self , span_range : SpanRange , msg : String) -> Self { self . children . push ((span_range , msg)) ; self } fn span_range_help (mut self , span_range : SpanRange , msg : String) -> Self { self . suggestions . push ((SuggestionKind :: Help , msg , Some (span_range))) ; self } fn span_range_note (mut self , span_range : SpanRange , msg : String) -> Self { self . suggestions . push ((SuggestionKind :: Note , msg , Some (span_range))) ; self } }
};
}
