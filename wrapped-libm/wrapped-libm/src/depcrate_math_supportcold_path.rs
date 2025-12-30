// Generated macro for cold_path (function)
macro_rules! Depcrate_math_supportcold_path {
() => {
// Module: crate::math::support
// Provides: {"cold_path"}
// Dependencies: {}
# [doc = " Hint to the compiler that the current path is cold."] pub fn cold_path () { # [cfg (intrinsics_enabled)] core :: intrinsics :: cold_path () ; }
};
}
