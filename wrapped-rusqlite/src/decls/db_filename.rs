macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! db_filename {
    () => {
        deps!();
        # [inline] pub (crate) unsafe fn db_filename < N : Name > (_ : std :: marker :: PhantomData < & () > , ptr : * mut ffi :: sqlite3 , db_name : N ,) -> Option < & str > { let db_name = db_name . as_cstr () . unwrap () ; let db_filename = ffi :: sqlite3_db_filename (ptr , db_name . as_ptr ()) ; if db_filename . is_null () { None } else { CStr :: from_ptr (db_filename) . to_str () . ok () } }
    };
}

db_filename!()