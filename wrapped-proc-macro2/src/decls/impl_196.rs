macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl FromIterator < TokenStream > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenStream > > (streams : I) -> Self { TokenStream :: _new (streams . into_iter () . map (| i | i . inner) . collect ()) } }
    };
}

impl_196!();