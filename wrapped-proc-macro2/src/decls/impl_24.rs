macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Extend < TokenStream > for TokenStream { fn extend < I : IntoIterator < Item = TokenStream > > (& mut self , streams : I) { self . inner . extend (streams . into_iter () . map (| stream | stream . inner)) ; } }
    };
}

impl_24!()