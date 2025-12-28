macro_rules! deps {
    () => {
        SpanLike!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl SpanLike for usize { # [inline (always)] fn into_span (self) -> Option < Range < usize > > { Some (self .. self + 1) } }
    };
}

impl_115!()