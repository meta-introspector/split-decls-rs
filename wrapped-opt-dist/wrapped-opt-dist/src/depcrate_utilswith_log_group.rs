// Generated macro for with_log_group (function)
macro_rules! Depcrate_utilswith_log_group {
() => {
// Module: crate::utils
// Provides: {"with_log_group"}
// Dependencies: {}
# [doc = " Wraps all output produced within the `func` closure in a CI output group, if we're running in"] # [doc = " CI."] pub fn with_log_group < F : FnOnce () -> R , R > (group : & str , func : F) -> R { if is_in_ci () { println ! ("::group::{group}") ; let result = func () ; println ! ("::endgroup::") ; result } else { func () } }
};
}
