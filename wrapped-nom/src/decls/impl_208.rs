macro_rules! deps {
    () => {
        NilError!();
        ErrorKind!();
        ParseError!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < I > ParseError < I > for NilError { fn from_error_kind (_ : I , _ : ErrorKind) -> NilError { NilError } fn append (_ : I , _ : ErrorKind , _ : NilError) -> NilError { NilError } }
    };
}

impl_208!()