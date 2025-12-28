macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl From < & str > for TokenStream { fn from (tokens : & str) -> Self { Self (tokens . to_string ()) } }
    };
}

impl_135!()