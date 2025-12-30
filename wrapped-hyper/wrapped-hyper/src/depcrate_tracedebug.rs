// Generated macro for debug (macro)
macro_rules! Depcrate_tracedebug {
() => {
// Module: crate::trace
// Provides: {"debug"}
// Dependencies: {}
macro_rules ! debug { ($ ($ arg : tt) +) => { # [cfg (feature = "tracing")] { tracing :: debug ! ($ ($ arg) +) ; } } }
};
}
