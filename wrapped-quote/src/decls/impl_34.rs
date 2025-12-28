macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl ToTokens for u16 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: u16_suffixed (* self)) ; } }
    };
}

impl_34!()