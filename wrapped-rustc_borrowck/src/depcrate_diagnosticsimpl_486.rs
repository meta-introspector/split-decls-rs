// Generated macro for impl_486 (impl)
macro_rules! Depcrate_diagnosticsimpl_486 {
() => {
// Module: crate::diagnostics
// Provides: {"impl_486"}
// Dependencies: {}
impl < 'infcx , 'tcx > BorrowckDiagnosticsBuffer < 'infcx , 'tcx > { pub (crate) fn buffer_non_error (& mut self , diag : Diag < 'infcx , () >) { self . buffered_diags . push (BufferedDiag :: NonError (diag)) ; } }
};
}
