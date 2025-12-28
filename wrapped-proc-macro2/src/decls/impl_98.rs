macro_rules! deps {
    () => {
        RcVecBuilder!();
        TokenStream!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl FromIterator < TokenStream > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenStream > > (streams : I) -> Self { let mut v = RcVecBuilder :: new () ; for stream in streams { v . extend (stream . take_inner ()) ; } TokenStream { inner : v . build () } } }
    };
}

impl_98!();