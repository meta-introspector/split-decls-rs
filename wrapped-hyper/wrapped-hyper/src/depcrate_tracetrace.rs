// Generated macro for trace (macro)
macro_rules! Depcrate_tracetrace {
() => {
// Module: crate::trace
// Provides: {"trace"}
// Dependencies: {}
macro_rules ! trace { ($ ($ arg : tt) *) => { # [cfg (feature = "tracing")] { tracing :: trace ! ($ ($ arg) +) ; } } }
};
}
