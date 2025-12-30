// Generated macro for libdir (function)
macro_rules! Depcrate_utils_helperslibdir {
() => {
// Module: crate::utils::helpers
// Provides: {"libdir"}
// Dependencies: {}
# [doc = " Returns the corresponding relative library directory that the compiler's"] # [doc = " dylibs will be found in."] pub fn libdir (target : TargetSelection) -> & 'static str { if target . is_windows () || target . contains ("cygwin") { "bin" } else { "lib" } }
};
}
