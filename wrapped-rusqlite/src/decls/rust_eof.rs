macro_rules! deps {
    () => {
        VTabCursor!();
    };
}

macro_rules! rust_eof {
    () => {
        deps!();
        unsafe extern "C" fn rust_eof < C > (cursor : * mut sqlite3_vtab_cursor) -> c_int where C : VTabCursor , { let cr = cursor . cast :: < C > () ; (* cr) . eof () as c_int }
    };
}

rust_eof!();