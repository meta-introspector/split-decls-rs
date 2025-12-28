macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl ToTokens for String { fn to_tokens (& self , tokens : & mut TokenStream) { self . as_str () . to_tokens (tokens) ; } }
    };
}

impl_26!()