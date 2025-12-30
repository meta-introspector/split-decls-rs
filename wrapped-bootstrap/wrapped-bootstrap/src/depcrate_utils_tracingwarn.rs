// Generated macro for warn (macro)
macro_rules! Depcrate_utils_tracingwarn {
() => {
// Module: crate::utils::tracing
// Provides: {"warn"}
// Dependencies: {}
# [macro_export] macro_rules ! warn { ($ ($ tokens : tt) *) => { # [cfg (feature = "tracing")] :: tracing :: warn ! ($ ($ tokens) *) } }
};
}
