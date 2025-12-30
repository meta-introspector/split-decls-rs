// Generated macro for trace_span (macro)
macro_rules! Depcrate_tracetrace_span {
() => {
// Module: crate::trace
// Provides: {"trace_span"}
// Dependencies: {}
macro_rules ! trace_span { ($ ($ arg : tt) *) => { { # [cfg (feature = "tracing")] { let _span = tracing :: trace_span ! ($ ($ arg) +) ; _span . entered () } } } }
};
}
