macro_rules! deps {
    () => {
        Input!();
        Error!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        # [doc = " Mandatory [`ParseError`] implementation."] impl < 'a > ParseError < Input < 'a > > for Error < 'a > { fn from_error_kind (input : Input < 'a > , kind : ErrorKind) -> Self { Error { input , code : kind , detail : None } } fn append (_ : Input < 'a > , _ : ErrorKind , other : Self) -> Self { other } }
    };
}

impl_96!()