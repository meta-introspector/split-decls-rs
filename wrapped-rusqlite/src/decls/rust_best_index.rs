macro_rules! deps {
    () => {
        VTab!();
        IndexInfo!();
        Error!();
    };
}

macro_rules! rust_best_index {
    () => {
        deps!();
        unsafe extern "C" fn rust_best_index < 'vtab , T > (vtab : * mut sqlite3_vtab , info : * mut ffi :: sqlite3_index_info ,) -> c_int where T : VTab < 'vtab > , { let vt = vtab . cast :: < T > () ; let mut idx_info = IndexInfo (info) ; match (* vt) . best_index (& mut idx_info) { Ok (_) => ffi :: SQLITE_OK , Err (Error :: SqliteFailure (err , s)) => { if let Some (err_msg) = s { set_err_msg (vtab , & err_msg) ; } err . extended_code } Err (err) => { set_err_msg (vtab , & err . to_string ()) ; ffi :: SQLITE_ERROR } } }
    };
}

rust_best_index!();