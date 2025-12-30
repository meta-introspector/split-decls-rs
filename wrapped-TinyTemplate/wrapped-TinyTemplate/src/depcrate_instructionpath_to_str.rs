// Generated macro for path_to_str (function)
macro_rules! Depcrate_instructionpath_to_str {
() => {
// Module: crate::instruction
// Provides: {"path_to_str"}
// Dependencies: {}
# [doc = " Convert a path back into a dotted string."] pub (crate) fn path_to_str (path : PathSlice) -> String { let mut path_str = "" . to_string () ; for (i , step) in path . iter () . enumerate () { if i > 0 { path_str . push ('.') ; } path_str . push_str (step) ; } path_str }
};
}
