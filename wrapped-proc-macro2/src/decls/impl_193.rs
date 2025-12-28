macro_rules! deps {
    () => {
        TokenStream!();
        TokenTree!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl Extend < TokenTree > for TokenStream { fn extend < I : IntoIterator < Item = TokenTree > > (& mut self , tokens : I) { self . inner . extend (tokens) ; } }
    };
}

impl_193!();