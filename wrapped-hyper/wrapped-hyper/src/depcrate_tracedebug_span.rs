// Generated macro for debug_span (macro)
macro_rules! Depcrate_tracedebug_span {
() => {
// Module: crate::trace
// Provides: {"debug_span"}
// Dependencies: {}
macro_rules ! debug_span { ($ ($ arg : tt) *) => { { # [cfg (feature = "tracing")] { let _span = tracing :: debug_span ! ($ ($ arg) +) ; _span . entered () } } } }
};
}
