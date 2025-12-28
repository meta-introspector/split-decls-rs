macro_rules! deps {
    () => {
        ParseError!();
        ErrorKind!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < I > ParseError < I > for (I , ErrorKind) { fn from_error_kind (input : I , kind : ErrorKind) -> Self { (input , kind) } fn append (_ : I , _ : ErrorKind , other : Self) -> Self { other } }
    };
}

impl_22!();