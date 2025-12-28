macro_rules! deps {
    () => {
        Input!();
        Error!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < 'a , E > FromExternalError < Input < 'a > , E > for Error < 'a > { fn from_external_error (input : Input < 'a > , kind : ErrorKind , _e : E) -> Self { Error { input , code : kind , detail : None } } }
    };
}

impl_97!();