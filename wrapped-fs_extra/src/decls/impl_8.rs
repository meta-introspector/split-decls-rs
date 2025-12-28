macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl From < StripPrefixError > for Error { fn from (err : StripPrefixError) -> Error { Error :: new (ErrorKind :: StripPrefix (err) , "StripPrefixError. Look inside for more details" ,) } }
    };
}

impl_8!();