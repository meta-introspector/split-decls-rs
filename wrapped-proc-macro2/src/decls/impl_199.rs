macro_rules! deps {
    () => {
        LexError!();
        Span!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl LexError { pub fn span (& self) -> Span { Span :: _new (self . inner . span ()) } }
    };
}

impl_199!()