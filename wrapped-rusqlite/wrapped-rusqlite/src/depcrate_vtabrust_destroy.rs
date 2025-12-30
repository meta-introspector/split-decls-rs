// Generated macro for rust_destroy (function)
macro_rules! Depcrate_vtabrust_destroy {
() => {
// Module: crate::vtab
// Provides: {"rust_destroy"}
// Dependencies: {}
unsafe extern "C" fn rust_destroy < 'vtab , T > (vtab : * mut sqlite3_vtab) -> c_int where T : CreateVTab < 'vtab > , { if vtab . is_null () { return ffi :: SQLITE_OK ; } let vt = vtab . cast :: < T > () ; match (* vt) . destroy () { Ok (_) => { drop (Box :: from_raw (vt)) ; ffi :: SQLITE_OK } Err (Error :: SqliteFailure (err , s)) => { if let Some (err_msg) = s { set_err_msg (vtab , & err_msg) ; } err . extended_code } Err (err) => { set_err_msg (vtab , & err . to_string ()) ; ffi :: SQLITE_ERROR } } }
};
}
