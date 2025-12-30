// Generated macro for fatal (macro)
macro_rules! Depcrate_diagnosticsfatal {
() => {
// Module: crate::diagnostics
// Provides: {"fatal"}
// Dependencies: {}
# [macro_export] macro_rules ! fatal { ($ ($ arg : tt) *) => { let status = :: colored :: Colorize :: bright_red ("FATAL: ") ; let status = :: colored :: Colorize :: bold (status) ; eprint ! ("{status}") ; eprintln ! ($ ($ arg) *) ; panic ! ("fatal error") ; } ; }
};
}
