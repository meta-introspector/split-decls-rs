macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl ToTokens for i32 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: i32_suffixed (* self)) ; } }
    };
}

impl_29!()