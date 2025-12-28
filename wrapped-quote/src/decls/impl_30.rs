macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl ToTokens for i64 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: i64_suffixed (* self)) ; } }
    };
}

impl_30!()