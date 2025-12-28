macro_rules! deps {
    () => {
        FromExternalError!();
        ErrorKind!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < I , E > FromExternalError < I , E > for (I , ErrorKind) { fn from_external_error (input : I , kind : ErrorKind , _e : E) -> Self { (input , kind) } }
    };
}

impl_24!()