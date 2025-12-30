// Generated macro for info_span (macro)
macro_rules! Depcrate_traceinfo_span {
() => {
// Module: crate::trace
// Provides: {"info_span"}
// Dependencies: {}
macro_rules ! info_span { ($ ($ arg : tt) *) => { { # [cfg (feature = "tracing")] { let _span = tracing :: info_span ! ($ ($ arg) +) ; _span . entered () } } } }
};
}
