// Generated macro for rust_rowid (function)
macro_rules! Depcrate_vtabrust_rowid {
() => {
// Module: crate::vtab
// Provides: {"rust_rowid"}
// Dependencies: {}
unsafe extern "C" fn rust_rowid < C > (cursor : * mut sqlite3_vtab_cursor , p_rowid : * mut ffi :: sqlite3_int64 ,) -> c_int where C : VTabCursor , { let cr = cursor . cast :: < C > () ; match (* cr) . rowid () { Ok (rowid) => { * p_rowid = rowid ; ffi :: SQLITE_OK } err => cursor_error (cursor , err) , } }
};
}
