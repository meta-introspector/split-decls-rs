macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! error_from_sqlite_code {
    () => {
        deps!();
        # [cold] pub fn error_from_sqlite_code (code : c_int , message : Option < String >) -> Error { Error :: SqliteFailure (ffi :: Error :: new (code) , message) }
    };
}

error_from_sqlite_code!();