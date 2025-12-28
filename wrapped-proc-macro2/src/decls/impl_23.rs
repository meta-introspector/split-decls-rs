macro_rules! deps {
    () => {
        TokenTree!();
        TokenStream!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Extend < TokenTree > for TokenStream { fn extend < I : IntoIterator < Item = TokenTree > > (& mut self , tokens : I) { self . inner . extend (tokens) ; } }
    };
}

impl_23!()