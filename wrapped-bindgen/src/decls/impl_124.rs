macro_rules! deps {
    () => {
        ToTokens!();
        TokenStream!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl ToTokens for String { fn to_tokens (& self , tokens : & mut TokenStream) { self . as_str () . to_tokens (tokens) ; } }
    };
}

impl_124!()