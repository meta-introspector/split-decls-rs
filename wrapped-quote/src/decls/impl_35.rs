macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl ToTokens for u32 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: u32_suffixed (* self)) ; } }
    };
}

impl_35!();