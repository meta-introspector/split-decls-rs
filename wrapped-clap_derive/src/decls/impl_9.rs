macro_rules! deps {
    () => {
        AttrValue!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl ToTokens for AttrValue { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Self :: LitStr (t) => t . to_tokens (tokens) , Self :: Expr (t) => t . to_tokens (tokens) , Self :: Call (t) => { let t = quote ! (# (# t) ,*) ; t . to_tokens (tokens) ; } } } }
    };
}

impl_9!();