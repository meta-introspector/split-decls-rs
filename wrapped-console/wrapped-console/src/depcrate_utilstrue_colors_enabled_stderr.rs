// Generated macro for true_colors_enabled_stderr (function)
macro_rules! Depcrate_utilstrue_colors_enabled_stderr {
() => {
// Module: crate::utils
// Provides: {"true_colors_enabled_stderr"}
// Dependencies: {}
# [doc = " Returns `true` if true colors should be enabled for stderr."] # [inline] pub fn true_colors_enabled_stderr () -> bool { STDERR_TRUE_COLORS . load (Ordering :: Relaxed) }
};
}
