macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl ToTokens for TokenStream { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . extend (iter :: once (self . clone ())) ; } fn into_token_stream (self) -> TokenStream { self } }
    };
}

impl_50!();