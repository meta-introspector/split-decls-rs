macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_451 {
    () => {
        deps!();
        impl From < Range < usize > > for Span { # [inline] fn from (range : Range < usize >) -> Span { Span { start : range . start , end : range . end } } }
    };
}

impl_451!();