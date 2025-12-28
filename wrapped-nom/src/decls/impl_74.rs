macro_rules! deps {
    () => {
        ParseError!();
        ErrorKind!();
        CustomError!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < I > ParseError < I > for CustomError { fn from_error_kind (_ : I , _ : ErrorKind) -> Self { CustomError } fn append (_ : I , _ : ErrorKind , _ : CustomError) -> Self { CustomError } }
    };
}

impl_74!()