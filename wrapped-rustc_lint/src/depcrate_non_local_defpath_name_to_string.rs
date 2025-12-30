// Generated macro for path_name_to_string (function)
macro_rules! Depcrate_non_local_defpath_name_to_string {
() => {
// Module: crate::non_local_def
// Provides: {"path_name_to_string"}
// Dependencies: {}
# [doc = " Return a \"error message-able\" ident for the last segment of the `Path`"] fn path_name_to_string (path : & Path < '_ >) -> String { path . segments . last () . unwrap () . ident . to_string () }
};
}
