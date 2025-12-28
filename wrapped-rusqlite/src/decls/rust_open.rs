macro_rules! deps {
    () => {
        Error!();
        VTab!();
    };
}

macro_rules! rust_open {
    () => {
        deps!();
        unsafe extern "C" fn rust_open < 'vtab , T > (vtab : * mut sqlite3_vtab , pp_cursor : * mut * mut sqlite3_vtab_cursor ,) -> c_int where T : VTab < 'vtab > + 'vtab , { let vt = vtab . cast :: < T > () ; match (* vt) . open () { Ok (cursor) => { let boxed_cursor : * mut T :: Cursor = Box :: into_raw (Box :: new (cursor)) ; * pp_cursor = boxed_cursor . cast :: < sqlite3_vtab_cursor > () ; ffi :: SQLITE_OK } Err (Error :: SqliteFailure (err , s)) => { if let Some (err_msg) = s { set_err_msg (vtab , & err_msg) ; } err . extended_code } Err (err) => { set_err_msg (vtab , & err . to_string ()) ; ffi :: SQLITE_ERROR } } }
    };
}

rust_open!();