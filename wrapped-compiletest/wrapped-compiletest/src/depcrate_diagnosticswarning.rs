// Generated macro for warning (macro)
macro_rules! Depcrate_diagnosticswarning {
() => {
// Module: crate::diagnostics
// Provides: {"warning"}
// Dependencies: {}
# [macro_export] macro_rules ! warning { ($ ($ arg : tt) *) => { let status = :: colored :: Colorize :: yellow ("WARNING: ") ; let status = :: colored :: Colorize :: bold (status) ; eprint ! ("{status}") ; eprintln ! ($ ($ arg) *) ; } ; }
};
}
