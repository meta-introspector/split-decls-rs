macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl Extend < TokenStream > for TokenStream { fn extend < I : IntoIterator < Item = TokenStream > > (& mut self , streams : I) { self . inner . make_mut () . extend (streams . into_iter () . flatten ()) ; } }
    };
}

impl_100!();