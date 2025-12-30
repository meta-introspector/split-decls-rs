// Generated macro for is_debug_info (function)
macro_rules! Depcrate_utils_helpersis_debug_info {
() => {
// Module: crate::utils::helpers
// Provides: {"is_debug_info"}
// Dependencies: {}
# [doc = " Returns `true` if the file name given looks like a debug info file"] pub fn is_debug_info (name : & str) -> bool { name . ends_with (".pdb") }
};
}
