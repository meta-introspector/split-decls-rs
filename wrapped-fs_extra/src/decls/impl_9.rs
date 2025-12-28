macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl From < OsString > for Error { fn from (err : OsString) -> Error { Error :: new (ErrorKind :: OsString (err) , "OsString. Look inside for more details" ,) } }
    };
}

impl_9!()