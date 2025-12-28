macro_rules! deps {
    () => {
        Values!();
        VTabCursor!();
        Filters!();
    };
}

macro_rules! rust_filter {
    () => {
        deps!();
        unsafe extern "C" fn rust_filter < C > (cursor : * mut sqlite3_vtab_cursor , idx_num : c_int , idx_str : * const c_char , argc : c_int , argv : * mut * mut ffi :: sqlite3_value ,) -> c_int where C : VTabCursor , { use std :: str ; let idx_name = if idx_str . is_null () { None } else { let c_slice = CStr :: from_ptr (idx_str) . to_bytes () ; Some (str :: from_utf8_unchecked (c_slice)) } ; let args = slice :: from_raw_parts_mut (argv , argc as usize) ; let values = Values { args } ; let cr = cursor as * mut C ; cursor_error (cursor , (* cr) . filter (idx_num , idx_name , & Filters { values })) }
    };
}

rust_filter!()