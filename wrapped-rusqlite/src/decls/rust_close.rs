macro_rules! deps {
    () => {
        VTabCursor!();
    };
}

macro_rules! rust_close {
    () => {
        deps!();
        unsafe extern "C" fn rust_close < C > (cursor : * mut sqlite3_vtab_cursor) -> c_int where C : VTabCursor , { let cr = cursor . cast :: < C > () ; drop (Box :: from_raw (cr)) ; ffi :: SQLITE_OK }
    };
}

rust_close!();