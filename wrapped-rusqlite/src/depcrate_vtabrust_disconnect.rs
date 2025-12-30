// Generated macro for rust_disconnect (function)
macro_rules! Depcrate_vtabrust_disconnect {
() => {
// Module: crate::vtab
// Provides: {"rust_disconnect"}
// Dependencies: {}
unsafe extern "C" fn rust_disconnect < 'vtab , T > (vtab : * mut sqlite3_vtab) -> c_int where T : VTab < 'vtab > , { if vtab . is_null () { return ffi :: SQLITE_OK ; } let vtab = vtab . cast :: < T > () ; drop (Box :: from_raw (vtab)) ; ffi :: SQLITE_OK }
};
}
