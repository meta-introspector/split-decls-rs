// Generated macro for span (macro)
macro_rules! Depcrate_tracespan {
() => {
// Module: crate::trace
// Provides: {"span"}
// Dependencies: {}
macro_rules ! span { ($ ($ arg : tt) *) => { { # [cfg (feature = "tracing")] { let _span = tracing :: span ! ($ ($ arg) +) ; _span . entered () } } } }
};
}
