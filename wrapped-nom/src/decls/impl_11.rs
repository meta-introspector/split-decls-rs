macro_rules! deps {
    () => {
        Error!();
        ParseError!();
        ErrorKind!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < I > ParseError < I > for Error < I > { fn from_error_kind (input : I , kind : ErrorKind) -> Self { Error { input , code : kind } } fn append (_ : I , _ : ErrorKind , other : Self) -> Self { other } }
    };
}

impl_11!()