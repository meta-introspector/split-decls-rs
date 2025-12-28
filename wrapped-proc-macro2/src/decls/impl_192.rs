macro_rules! deps {
    () => {
        TokenTree!();
        TokenStream!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl From < TokenTree > for TokenStream { fn from (token : TokenTree) -> Self { TokenStream :: _new (imp :: TokenStream :: from (token)) } }
    };
}

impl_192!();