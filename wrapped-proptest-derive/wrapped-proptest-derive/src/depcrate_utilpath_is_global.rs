// Generated macro for path_is_global (function)
macro_rules! Depcrate_utilpath_is_global {
() => {
// Module: crate::util
// Provides: {"path_is_global"}
// Dependencies: {}
# [doc = " Does the path have a leading `::`?"] pub fn path_is_global (path : & syn :: Path) -> bool { path . leading_colon . is_some () }
};
}
