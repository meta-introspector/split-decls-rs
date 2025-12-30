// Generated macro for error_span (macro)
macro_rules! Depcrate_traceerror_span {
() => {
// Module: crate::trace
// Provides: {"error_span"}
// Dependencies: {}
macro_rules ! error_span { ($ ($ arg : tt) *) => { { # [cfg (feature = "tracing")] { let _span = tracing :: error_span ! ($ ($ arg) +) ; _span . entered () } } } }
};
}
