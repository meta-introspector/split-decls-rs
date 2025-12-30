// Generated macro for debug (macro)
macro_rules! Depcrate_utils_tracingdebug {
() => {
// Module: crate::utils::tracing
// Provides: {"debug"}
// Dependencies: {}
# [macro_export] macro_rules ! debug { ($ ($ tokens : tt) *) => { # [cfg (feature = "tracing")] :: tracing :: debug ! ($ ($ tokens) *) } }
};
}
