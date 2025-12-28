macro_rules! deps {
    () => {
        TokenStream!();
        TokenTree!();
        RcVecBuilder!();
        TokenStreamBuilder!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl TokenStreamBuilder { pub (crate) fn new () -> Self { TokenStreamBuilder { inner : RcVecBuilder :: new () , } } pub (crate) fn with_capacity (cap : usize) -> Self { TokenStreamBuilder { inner : RcVecBuilder :: with_capacity (cap) , } } pub (crate) fn push_token_from_parser (& mut self , tt : TokenTree) { self . inner . push (tt) ; } pub (crate) fn build (self) -> TokenStream { TokenStream { inner : self . inner . build () , } } }
    };
}

impl_88!();