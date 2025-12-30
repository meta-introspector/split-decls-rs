// Generated macro for set_colors_enabled (function)
macro_rules! Depcrate_utilsset_colors_enabled {
() => {
// Module: crate::utils
// Provides: {"set_colors_enabled"}
// Dependencies: {}
# [doc = " Forces colorization on or off for stdout."] # [doc = ""] # [doc = " This overrides the default for the current process and changes the return value of the"] # [doc = " `colors_enabled` function."] # [inline] pub fn set_colors_enabled (val : bool) { STDOUT_COLORS . store (val , Ordering :: Relaxed) }
};
}
