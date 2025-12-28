macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl Spanned for DelimSpan { fn __span (& self) -> Span { self . join () } }
    };
}

impl_55!()