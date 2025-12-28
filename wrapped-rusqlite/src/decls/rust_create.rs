macro_rules! deps {
    () => {
        CreateVTab!();
        VTabConnection!();
    };
}

macro_rules! rust_create {
    () => {
        deps!();
        unsafe extern "C" fn rust_create < 'vtab , T > (db : * mut ffi :: sqlite3 , aux : * mut c_void , argc : c_int , argv : * const * const c_char , pp_vtab : * mut * mut sqlite3_vtab , err_msg : * mut * mut c_char ,) -> c_int where T : CreateVTab < 'vtab > , { let mut conn = VTabConnection (db) ; let aux = aux . cast :: < T :: Aux > () ; let args = slice :: from_raw_parts (argv , argc as usize) ; let vec = args . iter () . map (| & cs | CStr :: from_ptr (cs) . to_bytes ()) . collect :: < Vec < _ > > () ; match T :: create (& mut conn , aux . as_ref () , & vec [..]) { Ok ((sql , vtab)) => match std :: ffi :: CString :: new (sql) { Ok (c_sql) => { let rc = ffi :: sqlite3_declare_vtab (db , c_sql . as_ptr ()) ; if rc == ffi :: SQLITE_OK { let boxed_vtab : * mut T = Box :: into_raw (Box :: new (vtab)) ; * pp_vtab = boxed_vtab . cast :: < sqlite3_vtab > () ; ffi :: SQLITE_OK } else { let err = error_from_sqlite_code (rc , None) ; to_sqlite_error (& err , err_msg) } } Err (err) => { * err_msg = alloc (& err . to_string ()) ; ffi :: SQLITE_ERROR } } , Err (err) => to_sqlite_error (& err , err_msg) , } }
    };
}

rust_create!()