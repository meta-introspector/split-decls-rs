// Generated macro for set_true_colors_enabled (function)
macro_rules! Depcrate_utilsset_true_colors_enabled {
() => {
// Module: crate::utils
// Provides: {"set_true_colors_enabled"}
// Dependencies: {}
# [doc = " Forces true colorization on or off for stdout."] # [doc = ""] # [doc = " This overrides the default for the current process and changes the return value of the"] # [doc = " `true_colors_enabled` function."] # [inline] pub fn set_true_colors_enabled (val : bool) { STDOUT_TRUE_COLORS . store (val , Ordering :: Relaxed) }
};
}
