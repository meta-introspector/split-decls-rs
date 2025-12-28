macro_rules! Spanned {
    () => {
        pub trait Spanned : private :: Sealed { fn __span (& self) -> Span ; }
    };
}

Spanned!()