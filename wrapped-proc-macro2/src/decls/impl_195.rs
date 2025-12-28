macro_rules! deps {
    () => {
        TokenTree!();
        TokenStream!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        # [doc = " Collects a number of token trees into a single stream."] impl FromIterator < TokenTree > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenTree > > (tokens : I) -> Self { TokenStream :: _new (tokens . into_iter () . collect ()) } }
    };
}

impl_195!();