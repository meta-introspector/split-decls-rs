macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Error { # [doc = " Returns the underlying SQLite error if this is [`Error::SqliteFailure`]."] # [inline] # [must_use] pub fn sqlite_error (& self) -> Option < & ffi :: Error > { match self { Self :: SqliteFailure (error , _) => Some (error) , _ => None , } } # [doc = " Returns the underlying SQLite error code if this is"] # [doc = " [`Error::SqliteFailure`]."] # [inline] # [must_use] pub fn sqlite_error_code (& self) -> Option < ffi :: ErrorCode > { self . sqlite_error () . map (| error | error . code) } }
    };
}

impl_9!()