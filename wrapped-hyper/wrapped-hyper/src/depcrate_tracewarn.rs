// Generated macro for warn (macro)
macro_rules! Depcrate_tracewarn {
() => {
// Module: crate::trace
// Provides: {"warn"}
// Dependencies: {}
macro_rules ! warn { ($ ($ arg : tt) *) => { # [cfg (feature = "tracing")] { tracing :: warn ! ($ ($ arg) +) ; } } }
};
}
