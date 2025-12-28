macro_rules! deps {
    () => {
        Span!();
        LexError!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl LexError { pub (crate) fn span (& self) -> Span { self . span } pub (crate) fn call_site () -> Self { LexError { span : Span :: call_site () , } } }
    };
}

impl_83!()