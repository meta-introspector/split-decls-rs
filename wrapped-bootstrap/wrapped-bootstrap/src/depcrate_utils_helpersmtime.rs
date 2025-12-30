// Generated macro for mtime (function)
macro_rules! Depcrate_utils_helpersmtime {
() => {
// Module: crate::utils::helpers
// Provides: {"mtime"}
// Dependencies: {}
# [doc = " Returns the last-modified time for `path`, or zero if it doesn't exist."] pub fn mtime (path : & Path) -> SystemTime { fs :: metadata (path) . and_then (| f | f . modified ()) . unwrap_or (UNIX_EPOCH) }
};
}
