// Generated macro for assert_no_fragment (function)
macro_rules! Depcrate_utilsassert_no_fragment {
() => {
// Module: crate::utils
// Provides: {"assert_no_fragment"}
// Dependencies: {}
pub (crate) fn assert_no_fragment (path : & str) { if path . contains ('#') { # [cfg (not (target_os = "wasi"))] throw_str ("You cannot use fragments (hash) in memory history.") ; # [cfg (target_os = "wasi")] panic ! ("You cannot use fragments (hash) in memory history.") ; } }
};
}
