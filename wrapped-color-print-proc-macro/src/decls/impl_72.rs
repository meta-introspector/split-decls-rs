macro_rules! deps {
    () => {
        SpanError!();
        Error!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < Error > for SpanError { fn from (err : Error) -> SpanError { SpanError { err , span : None } } }
    };
}

impl_72!();