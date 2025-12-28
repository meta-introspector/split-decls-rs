macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl From < String > for TokenStream { fn from (tokens : String) -> Self { Self (tokens) } }
    };
}

impl_133!();