macro_rules! deps {
    () => {
        MultiSpan!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl From < Span > for MultiSpan { fn from (span : Span) -> MultiSpan { MultiSpan :: from_span (span) } }
    };
}

impl_54!();