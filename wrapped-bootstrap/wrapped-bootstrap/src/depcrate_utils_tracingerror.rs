// Generated macro for error (macro)
macro_rules! Depcrate_utils_tracingerror {
() => {
// Module: crate::utils::tracing
// Provides: {"error"}
// Dependencies: {}
# [macro_export] macro_rules ! error { ($ ($ tokens : tt) *) => { # [cfg (feature = "tracing")] :: tracing :: error ! ($ ($ tokens) *) } }
};
}
