macro_rules! deps {
    () => {
        Values!();
        ValueRef!();
        UpdateVTab!();
        Inserts!();
        Updates!();
        Error!();
    };
}

macro_rules! rust_update {
    () => {
        deps!();
        unsafe extern "C" fn rust_update < 'vtab , T > (vtab : * mut sqlite3_vtab , argc : c_int , argv : * mut * mut ffi :: sqlite3_value , p_rowid : * mut ffi :: sqlite3_int64 ,) -> c_int where T : UpdateVTab < 'vtab > + 'vtab , { assert ! (argc >= 1) ; let args = slice :: from_raw_parts_mut (argv , argc as usize) ; let vt = vtab . cast :: < T > () ; let r = if args . len () == 1 { (* vt) . delete (ValueRef :: from_value (args [0])) } else if ffi :: sqlite3_value_type (args [0]) == ffi :: SQLITE_NULL { let values = Values { args } ; match (* vt) . insert (& Inserts { values }) { Ok (rowid) => { * p_rowid = rowid ; Ok (()) } Err (e) => Err (e) , } } else { let values = Values { args } ; (* vt) . update (& Updates { values }) } ; match r { Ok (_) => ffi :: SQLITE_OK , Err (Error :: SqliteFailure (err , s)) => { if let Some (err_msg) = s { set_err_msg (vtab , & err_msg) ; } err . extended_code } Err (err) => { set_err_msg (vtab , & err . to_string ()) ; ffi :: SQLITE_ERROR } } }
    };
}

rust_update!()