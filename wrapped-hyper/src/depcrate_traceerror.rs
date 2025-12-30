// Generated macro for error (macro)
macro_rules! Depcrate_traceerror {
() => {
// Module: crate::trace
// Provides: {"error"}
// Dependencies: {}
macro_rules ! error { ($ ($ arg : tt) *) => { # [cfg (feature = "tracing")] { tracing :: error ! ($ ($ arg) +) ; } } }
};
}
