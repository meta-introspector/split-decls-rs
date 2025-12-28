macro_rules! SpanLike {
    () => {
        pub (crate) trait SpanLike { fn into_span (self) -> Option < Range < usize > > ; }
    };
}

SpanLike!();