macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl Spanned for Span { fn __span (& self) -> Span { * self } }
    };
}

impl_54!();