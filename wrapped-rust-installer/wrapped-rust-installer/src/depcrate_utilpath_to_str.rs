// Generated macro for path_to_str (function)
macro_rules! Depcrate_utilpath_to_str {
() => {
// Module: crate::util
// Provides: {"path_to_str"}
// Dependencies: {}
# [doc = " Converts a `&Path` to a UTF-8 `&str`."] pub fn path_to_str (path : & Path) -> Result < & str > { path . to_str () . ok_or_else (| | format_err ! ("path is not valid UTF-8 '{}'" , path . display ())) }
};
}
