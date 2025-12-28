macro_rules! deps {
    () => {
        VTabCursor!();
    };
}

macro_rules! rust_next {
    () => {
        deps!();
        unsafe extern "C" fn rust_next < C > (cursor : * mut sqlite3_vtab_cursor) -> c_int where C : VTabCursor , { let cr = cursor as * mut C ; cursor_error (cursor , (* cr) . next ()) }
    };
}

rust_next!()