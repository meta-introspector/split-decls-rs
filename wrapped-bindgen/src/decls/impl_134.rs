macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl From < & String > for TokenStream { fn from (tokens : & String) -> Self { Self (tokens . to_string ()) } }
    };
}

impl_134!()