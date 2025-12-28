macro_rules! deps {
    () => {
        SpanLike!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl SpanLike for Range < usize > { # [inline (always)] fn into_span (self) -> Option < Range < usize > > { Some (self) } }
    };
}

impl_114!()