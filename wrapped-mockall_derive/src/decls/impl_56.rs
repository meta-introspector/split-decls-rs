macro_rules! deps {
    () => {
        MockItem!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl ToTokens for MockItem { fn to_tokens (& self , tokens : & mut TokenStream) { match self { MockItem :: Module (mod_) => mod_ . to_tokens (tokens) , MockItem :: Struct (s) => s . to_tokens (tokens) } } }
    };
}

impl_56!();