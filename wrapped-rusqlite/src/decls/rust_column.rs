macro_rules! deps {
    () => {
        VTabCursor!();
        Context!();
    };
}

macro_rules! rust_column {
    () => {
        deps!();
        unsafe extern "C" fn rust_column < C > (cursor : * mut sqlite3_vtab_cursor , ctx : * mut ffi :: sqlite3_context , i : c_int ,) -> c_int where C : VTabCursor , { let cr = cursor . cast :: < C > () ; let mut ctxt = Context (ctx) ; result_error (ctx , (* cr) . column (& mut ctxt , i)) }
    };
}

rust_column!()