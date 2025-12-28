macro_rules! deps {
    () => {
        SpanError!();
        Error!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl SpanError { # [doc = " Creates a new `SpanError`."] pub fn new (err : Error , span : Option < Span >) -> Self { Self { err , span } } }
    };
}

impl_70!();