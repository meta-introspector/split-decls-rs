// Generated macro for SpanDedupDiagnostic (struct)
macro_rules! Depcrate_diagnosticsSpanDedupDiagnostic {
() => {
// Module: crate::diagnostics
// Provides: {"SpanDedupDiagnostic"}
// Dependencies: {}
# [doc = " Helps deduplicate a diagnostic to ensure it is only shown once per span."] pub struct SpanDedupDiagnostic (Mutex < FxHashSet < Span > >) ;
};
}
