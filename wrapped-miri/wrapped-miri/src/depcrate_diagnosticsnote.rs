// Generated macro for note (macro)
macro_rules! Depcrate_diagnosticsnote {
() => {
// Module: crate::diagnostics
// Provides: {"note"}
// Dependencies: {}
# [doc = " Generate a note/help text without a span."] macro_rules ! note { ($ ($ tt : tt) *) => { (None , format ! ($ ($ tt) *)) } ; }
};
}
