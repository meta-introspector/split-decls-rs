macro_rules! deps {
    () => {
        SpanLike!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl SpanLike for Option < Range < usize > > { # [inline (always)] fn into_span (self) -> Option < Range < usize > > { self } }
    };
}

impl_113!()