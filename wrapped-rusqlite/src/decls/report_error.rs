macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! report_error {
    () => {
        deps!();
        unsafe fn report_error (ctx : * mut sqlite3_context , err : & Error) { if let Error :: SqliteFailure (ref err , ref s) = * err { ffi :: sqlite3_result_error_code (ctx , err . extended_code) ; if let Some (Ok (cstr)) = s . as_ref () . map (| s | str_to_cstring (s)) { ffi :: sqlite3_result_error (ctx , cstr . as_ptr () , - 1) ; } } else { ffi :: sqlite3_result_error_code (ctx , ffi :: SQLITE_CONSTRAINT_FUNCTION) ; if let Ok (cstr) = str_to_cstring (& err . to_string ()) { ffi :: sqlite3_result_error (ctx , cstr . as_ptr () , - 1) ; } } }
    };
}

report_error!();