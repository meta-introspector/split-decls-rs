macro_rules! aggregate_context {
    () => {
        unsafe fn aggregate_context < A > (ctx : * mut sqlite3_context , bytes : usize) -> Option < * mut * mut A > { let pac = ffi :: sqlite3_aggregate_context (ctx , bytes as c_int) as * mut * mut A ; if pac . is_null () { return None ; } Some (pac) }
    };
}

aggregate_context!()