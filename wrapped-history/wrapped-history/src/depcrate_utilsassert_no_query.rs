// Generated macro for assert_no_query (function)
macro_rules! Depcrate_utilsassert_no_query {
() => {
// Module: crate::utils
// Provides: {"assert_no_query"}
// Dependencies: {}
pub (crate) fn assert_no_query (path : & str) { if path . contains ('?') { # [cfg (not (target_os = "wasi"))] throw_str ("You cannot have query in path, try use a variant of this method with `_query`.") ; # [cfg (target_os = "wasi")] panic ! ("You cannot have query in path, try use a variant of this method with `_query`.") ; } }
};
}
