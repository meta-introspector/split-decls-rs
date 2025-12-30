// Generated macro for is_glob_pattern (function)
macro_rules! Depcrate_internal_globis_glob_pattern {
() => {
// Module: crate::internal::glob
// Provides: {"is_glob_pattern"}
// Dependencies: {}
# [doc = " Returns true if `s` contains glob wildcards."] pub fn is_glob_pattern (s : & str) -> bool { s . contains (['?' , '*']) }
};
}
