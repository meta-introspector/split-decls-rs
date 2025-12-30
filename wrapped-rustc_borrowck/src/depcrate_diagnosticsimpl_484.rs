// Generated macro for impl_484 (impl)
macro_rules! Depcrate_diagnosticsimpl_484 {
() => {
// Module: crate::diagnostics
// Provides: {"impl_484"}
// Dependencies: {}
impl < 'infcx > BufferedDiag < 'infcx > { fn sort_span (& self) -> Span { match self { BufferedDiag :: Error (diag) => diag . sort_span , BufferedDiag :: NonError (diag) => diag . sort_span , } } }
};
}
