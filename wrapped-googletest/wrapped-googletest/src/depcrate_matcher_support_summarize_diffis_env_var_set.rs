// Generated macro for is_env_var_set (function)
macro_rules! Depcrate_matcher_support_summarize_diffis_env_var_set {
() => {
// Module: crate::matcher_support::summarize_diff
// Provides: {"is_env_var_set"}
// Dependencies: {}
fn is_env_var_set (var : & 'static str) -> bool { std :: env :: var (var) . map (| s | ! s . is_empty ()) . unwrap_or (false) }
};
}
