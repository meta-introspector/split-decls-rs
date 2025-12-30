// Generated macro for warn_span (macro)
macro_rules! Depcrate_tracewarn_span {
() => {
// Module: crate::trace
// Provides: {"warn_span"}
// Dependencies: {}
macro_rules ! warn_span { ($ ($ arg : tt) *) => { { # [cfg (feature = "tracing")] { let _span = tracing :: warn_span ! ($ ($ arg) +) ; _span . entered () } } } }
};
}
