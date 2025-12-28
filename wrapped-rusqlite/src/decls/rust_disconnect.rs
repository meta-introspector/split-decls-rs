macro_rules! deps {
    () => {
        VTab!();
    };
}

macro_rules! rust_disconnect {
    () => {
        deps!();
        unsafe extern "C" fn rust_disconnect < 'vtab , T > (vtab : * mut sqlite3_vtab) -> c_int where T : VTab < 'vtab > , { if vtab . is_null () { return ffi :: SQLITE_OK ; } let vtab = vtab . cast :: < T > () ; drop (Box :: from_raw (vtab)) ; ffi :: SQLITE_OK }
    };
}

rust_disconnect!()