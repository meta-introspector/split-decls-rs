// Generated macro for info (macro)
macro_rules! Depcrate_utils_tracinginfo {
() => {
// Module: crate::utils::tracing
// Provides: {"info"}
// Dependencies: {}
# [macro_export] macro_rules ! info { ($ ($ tokens : tt) *) => { # [cfg (feature = "tracing")] :: tracing :: info ! ($ ($ tokens) *) } }
};
}
