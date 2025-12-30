// Generated macro for true_colors_enabled (function)
macro_rules! Depcrate_utilstrue_colors_enabled {
() => {
// Module: crate::utils
// Provides: {"true_colors_enabled"}
// Dependencies: {}
# [doc = " Returns `true` if true colors should be enabled for stdout."] # [inline] pub fn true_colors_enabled () -> bool { STDOUT_TRUE_COLORS . load (Ordering :: Relaxed) }
};
}
