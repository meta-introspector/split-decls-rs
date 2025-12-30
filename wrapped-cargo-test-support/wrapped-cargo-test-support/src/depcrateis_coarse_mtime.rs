// Generated macro for is_coarse_mtime (function)
macro_rules! Depcrateis_coarse_mtime {
() => {
// Module: crate
// Provides: {"is_coarse_mtime"}
// Dependencies: {}
# [doc = " Returns `true` if the local filesystem has low-resolution mtimes."] pub fn is_coarse_mtime () -> bool { cfg ! (emulate_second_only_system) || cfg ! (target_os = "macos") && is_ci () }
};
}
