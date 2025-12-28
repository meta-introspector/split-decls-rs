macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl ToTokens for u8 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: u8_suffixed (* self)) ; } }
    };
}

impl_33!();