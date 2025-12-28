macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl ToTokens for u64 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: u64_suffixed (* self)) ; } }
    };
}

impl_36!()