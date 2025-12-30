// Generated macro for rust_close (function)
macro_rules! Depcrate_vtabrust_close {
() => {
// Module: crate::vtab
// Provides: {"rust_close"}
// Dependencies: {}
unsafe extern "C" fn rust_close < C > (cursor : * mut sqlite3_vtab_cursor) -> c_int where C : VTabCursor , { let cr = cursor . cast :: < C > () ; drop (Box :: from_raw (cr)) ; ffi :: SQLITE_OK }
};
}
