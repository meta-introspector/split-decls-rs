macro_rules! deps {
    () => {
        Error!();
        SpanError!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < Error > for SpanError { fn from (err : Error) -> SpanError { SpanError { err , span : None } } }
    };
}

impl_72!()