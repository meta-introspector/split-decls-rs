// Generated macro for set_colors_enabled_stderr (function)
macro_rules! Depcrate_utilsset_colors_enabled_stderr {
() => {
// Module: crate::utils
// Provides: {"set_colors_enabled_stderr"}
// Dependencies: {}
# [doc = " Forces colorization on or off for stderr."] # [doc = ""] # [doc = " This overrides the default for the current process and changes the return value of the"] # [doc = " `colors_enabled_stderr` function."] # [inline] pub fn set_colors_enabled_stderr (val : bool) { STDERR_COLORS . store (val , Ordering :: Relaxed) }
};
}
