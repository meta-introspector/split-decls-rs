// Generated macro for info (macro)
macro_rules! Depcrate_traceinfo {
() => {
// Module: crate::trace
// Provides: {"info"}
// Dependencies: {}
macro_rules ! info { ($ ($ arg : tt) *) => { # [cfg (feature = "tracing")] { tracing :: info ! ($ ($ arg) +) ; } } }
};
}
