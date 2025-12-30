// Generated macro for eq_simple_path (function)
macro_rules! Depcrate_utileq_simple_path {
() => {
// Module: crate::util
// Provides: {"eq_simple_path"}
// Dependencies: {}
# [doc = " Returns true iff lhs matches the given simple Path."] pub fn eq_simple_path (mut lhs : & str , rhs : & syn :: Path) -> bool { if ! is_path_simple (rhs) { return false ; } if rhs . leading_colon . is_some () { if ! lhs . starts_with ("::") { return false ; } lhs = & lhs [2 ..] ; } eq_simple_pathseg (lhs , & rhs . segments) }
};
}
