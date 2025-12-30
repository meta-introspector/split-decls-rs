// Generated macro for rust_eof (function)
macro_rules! Depcrate_vtabrust_eof {
() => {
// Module: crate::vtab
// Provides: {"rust_eof"}
// Dependencies: {}
unsafe extern "C" fn rust_eof < C > (cursor : * mut sqlite3_vtab_cursor) -> c_int where C : VTabCursor , { let cr = cursor . cast :: < C > () ; (* cr) . eof () as c_int }
};
}
