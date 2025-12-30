// Generated macro for error (macro)
macro_rules! Depcrate_diagnosticserror {
() => {
// Module: crate::diagnostics
// Provides: {"error"}
// Dependencies: {}
# [macro_export] macro_rules ! error { ($ ($ arg : tt) *) => { let status = :: colored :: Colorize :: red ("ERROR: ") ; let status = :: colored :: Colorize :: bold (status) ; eprint ! ("{status}") ; eprintln ! ($ ($ arg) *) ; } ; }
};
}
