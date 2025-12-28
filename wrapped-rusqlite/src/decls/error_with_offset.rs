macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! error_with_offset {
    () => {
        deps!();
        # [cold] # [cfg (feature = "modern_sqlite")] pub unsafe fn error_with_offset (db : * mut ffi :: sqlite3 , code : c_int , sql : & str) -> Error { if db . is_null () { error_from_sqlite_code (code , None) } else { let error = ffi :: Error :: new (code) ; let msg = error_msg (db , code) ; if ffi :: ErrorCode :: Unknown == error . code { let offset = ffi :: sqlite3_error_offset (db) ; if offset >= 0 { return Error :: SqlInputError { error , msg : msg . unwrap_or ("error" . to_owned ()) , sql : sql . to_owned () , offset , } ; } } Error :: SqliteFailure (error , msg) } }
    };
}

error_with_offset!()