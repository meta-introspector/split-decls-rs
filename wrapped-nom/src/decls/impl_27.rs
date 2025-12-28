macro_rules! deps {
    () => {
        FromExternalError!();
        ErrorKind!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < I , E > FromExternalError < I , E > for () { fn from_external_error (_input : I , _kind : ErrorKind , _e : E) -> Self { } }
    };
}

impl_27!()