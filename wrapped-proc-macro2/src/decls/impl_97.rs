macro_rules! deps {
    () => {
        TokenTree!();
        TokenStream!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl FromIterator < TokenTree > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenTree > > (tokens : I) -> Self { let mut stream = TokenStream :: new () ; stream . extend (tokens) ; stream } }
    };
}

impl_97!()