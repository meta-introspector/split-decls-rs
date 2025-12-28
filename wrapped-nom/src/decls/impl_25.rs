macro_rules! deps {
    () => {
        ParseError!();
        ErrorKind!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < I > ParseError < I > for () { fn from_error_kind (_ : I , _ : ErrorKind) -> Self { } fn append (_ : I , _ : ErrorKind , _ : Self) -> Self { } }
    };
}

impl_25!()