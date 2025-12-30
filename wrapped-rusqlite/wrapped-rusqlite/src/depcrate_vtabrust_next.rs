// Generated macro for rust_next (function)
macro_rules! Depcrate_vtabrust_next {
() => {
// Module: crate::vtab
// Provides: {"rust_next"}
// Dependencies: {}
unsafe extern "C" fn rust_next < C > (cursor : * mut sqlite3_vtab_cursor) -> c_int where C : VTabCursor , { let cr = cursor as * mut C ; cursor_error (cursor , (* cr) . next ()) }
};
}
