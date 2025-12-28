macro_rules! deps {
    () => {
        TokenStream!();
        ToTokens!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl ToTokens for TokenStream { fn to_tokens (& self , dst : & mut TokenStream) { dst . combine (self) ; } fn into_token_stream (self) -> TokenStream { self } }
    };
}

impl_130!()