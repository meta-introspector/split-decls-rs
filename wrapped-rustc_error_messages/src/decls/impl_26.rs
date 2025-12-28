macro_rules! deps {
    () => {
        MultiSpan!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl From < Span > for MultiSpan { fn from (span : Span) -> MultiSpan { MultiSpan :: from_span (span) } }
    };
}

impl_26!()