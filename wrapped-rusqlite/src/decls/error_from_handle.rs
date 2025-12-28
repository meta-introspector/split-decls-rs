macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! error_from_handle {
    () => {
        deps!();
        # [cold] pub unsafe fn error_from_handle (db : * mut ffi :: sqlite3 , code : c_int) -> Error { error_from_sqlite_code (code , error_msg (db , code)) }
    };
}

error_from_handle!();