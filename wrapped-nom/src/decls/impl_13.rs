macro_rules! deps {
    () => {
        Error!();
        FromExternalError!();
        ErrorKind!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < I , E > FromExternalError < I , E > for Error < I > { # [doc = " Create a new error from an input position and an external error"] fn from_external_error (input : I , kind : ErrorKind , _e : E) -> Self { Error { input , code : kind } } }
    };
}

impl_13!();