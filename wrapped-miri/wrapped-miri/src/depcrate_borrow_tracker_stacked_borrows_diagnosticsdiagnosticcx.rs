// Generated macro for DiagnosticCx (struct)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_diagnosticsDiagnosticCx {
() => {
// Module: crate::borrow_tracker::stacked_borrows::diagnostics
// Provides: {"DiagnosticCx"}
// Dependencies: {}
pub struct DiagnosticCx < 'history , 'ecx , 'tcx > { operation : Operation , machine : & 'ecx MiriMachine < 'tcx > , history : & 'history mut AllocHistory , offset : Size , }
};
}
