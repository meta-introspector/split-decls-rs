macro_rules! deps {
    () => {
        TokenTree!();
        TokenStream!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl Extend < TokenTree > for TokenStream { fn extend < I : IntoIterator < Item = TokenTree > > (& mut self , tokens : I) { let mut vec = self . inner . make_mut () ; tokens . into_iter () . for_each (| token | push_token_from_proc_macro (vec . as_mut () , token)) ; } }
    };
}

impl_99!()