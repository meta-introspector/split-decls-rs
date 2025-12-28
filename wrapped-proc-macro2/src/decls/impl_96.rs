macro_rules! deps {
    () => {
        TokenStream!();
        RcVecBuilder!();
        TokenTree!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl From < TokenTree > for TokenStream { fn from (tree : TokenTree) -> Self { let mut stream = RcVecBuilder :: new () ; push_token_from_proc_macro (stream . as_mut () , tree) ; TokenStream { inner : stream . build () , } } }
    };
}

impl_96!()