macro_rules! deps {
    () => {
        TokenStream!();
        TokenTree!();
        Item!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        impl FromIterator < TokenTree > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenTree > > (iter : I) -> Self { TokenStream :: new (iter . into_iter () . collect :: < Vec < TokenTree > > ()) } }
    };
}

impl_448!()