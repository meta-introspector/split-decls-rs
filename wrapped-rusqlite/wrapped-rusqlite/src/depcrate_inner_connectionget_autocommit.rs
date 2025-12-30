// Generated macro for get_autocommit (function)
macro_rules! Depcrate_inner_connectionget_autocommit {
() => {
// Module: crate::inner_connection
// Provides: {"get_autocommit"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn get_autocommit (ptr : * mut ffi :: sqlite3) -> bool { ffi :: sqlite3_get_autocommit (ptr) != 0 }
};
}
