// Generated macro for create_diagnostics (function)
macro_rules! Depcrate_lowercreate_diagnostics {
() => {
// Module: crate::lower
// Provides: {"create_diagnostics"}
// Dependencies: {}
pub (crate) fn create_diagnostics (diagnostics : Vec < TyLoweringDiagnostic >) -> Diagnostics { (! diagnostics . is_empty ()) . then (| | ThinArc :: from_header_and_iter (() , diagnostics . into_iter ())) }
};
}
