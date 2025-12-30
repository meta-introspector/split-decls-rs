// Generated macro for help (macro)
macro_rules! Depcrate_diagnosticshelp {
() => {
// Module: crate::diagnostics
// Provides: {"help"}
// Dependencies: {}
# [macro_export] macro_rules ! help { ($ ($ arg : tt) *) => { let status = :: colored :: Colorize :: cyan ("HELP: ") ; let status = :: colored :: Colorize :: bold (status) ; eprint ! ("{status}") ; eprintln ! ($ ($ arg) *) ; } ; }
};
}
