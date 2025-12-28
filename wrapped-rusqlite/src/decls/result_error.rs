macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! result_error {
    () => {
        deps!();
        # [doc = " To raise an error, the `column` method should use this method to set the"] # [doc = " error message and return the error code."] # [cold] unsafe fn result_error < T > (ctx : * mut ffi :: sqlite3_context , result : Result < T >) -> c_int { match result { Ok (_) => ffi :: SQLITE_OK , Err (Error :: SqliteFailure (err , s)) => { match err . extended_code { ffi :: SQLITE_TOOBIG => { ffi :: sqlite3_result_error_toobig (ctx) ; } ffi :: SQLITE_NOMEM => { ffi :: sqlite3_result_error_nomem (ctx) ; } code => { ffi :: sqlite3_result_error_code (ctx , code) ; if let Some (Ok (cstr)) = s . map (| s | str_to_cstring (& s)) { ffi :: sqlite3_result_error (ctx , cstr . as_ptr () , - 1) ; } } } ; err . extended_code } Err (err) => { ffi :: sqlite3_result_error_code (ctx , ffi :: SQLITE_ERROR) ; if let Ok (cstr) = str_to_cstring (& err . to_string ()) { ffi :: sqlite3_result_error (ctx , cstr . as_ptr () , - 1) ; } ffi :: SQLITE_ERROR } } }
    };
}

result_error!()