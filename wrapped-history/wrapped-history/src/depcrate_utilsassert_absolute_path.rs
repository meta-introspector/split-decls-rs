// Generated macro for assert_absolute_path (function)
macro_rules! Depcrate_utilsassert_absolute_path {
() => {
// Module: crate::utils
// Provides: {"assert_absolute_path"}
// Dependencies: {}
pub (crate) fn assert_absolute_path (path : & str) { if ! path . starts_with ('/') { # [cfg (not (target_os = "wasi"))] throw_str ("You cannot use relative path with this history type.") ; # [cfg (target_os = "wasi")] panic ! ("You cannot use relative path with this history type.") ; } }
};
}
