// Generated macro for stdout_supports_color (function)
macro_rules! Depcrate_matcher_support_summarize_diffstdout_supports_color {
() => {
// Module: crate::matcher_support::summarize_diff
// Provides: {"stdout_supports_color"}
// Dependencies: {}
# [rustversion :: not (since (1.70))] fn stdout_supports_color () -> bool { is_env_var_set ("FORCE_COLOR") && ! is_env_var_set ("NO_COLOR") }
};
}
