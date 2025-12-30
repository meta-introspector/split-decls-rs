// Generated macro for path_to_storage (function)
macro_rules! Depcrate_utilspath_to_storage {
() => {
// Module: crate::utils
// Provides: {"path_to_storage"}
// Dependencies: {}
# [doc = " Converts a path into a string that can be persisted."] pub fn path_to_storage (path : & Path) -> String { # [cfg (windows)] { path . to_str () . unwrap () . replace ('\\' , "/") } # [cfg (not (windows))] { path . to_string_lossy () . into () } }
};
}
