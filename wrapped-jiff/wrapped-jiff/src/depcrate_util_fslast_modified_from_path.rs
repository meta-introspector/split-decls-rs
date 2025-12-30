// Generated macro for last_modified_from_path (function)
macro_rules! Depcrate_util_fslast_modified_from_path {
() => {
// Module: crate::util::fs
// Provides: {"last_modified_from_path"}
// Dependencies: {}
# [doc = " Returns the last modified time for the given file path as a Jiff timestamp."] # [doc = ""] # [doc = " If there was a problem accessing the last modified time or if it could not"] # [doc = " fit in a Jiff timestamp, then a warning message is logged and `None` is"] # [doc = " returned."] pub (crate) fn last_modified_from_path (path : & Path) -> Option < Timestamp > { let file = match File :: open (path) { Ok (file) => file , Err (_err) => { warn ! ("failed to open file to get last modified time {}: {_err}" , path . display () ,) ; return None ; } } ; last_modified_from_file (path , & file) }
};
}
