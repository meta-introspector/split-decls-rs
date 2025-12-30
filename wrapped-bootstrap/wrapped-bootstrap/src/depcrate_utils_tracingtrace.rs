// Generated macro for trace (macro)
macro_rules! Depcrate_utils_tracingtrace {
() => {
// Module: crate::utils::tracing
// Provides: {"trace"}
// Dependencies: {}
# [macro_export] macro_rules ! trace { ($ ($ tokens : tt) *) => { # [cfg (feature = "tracing")] :: tracing :: trace ! ($ ($ tokens) *) } }
};
}
