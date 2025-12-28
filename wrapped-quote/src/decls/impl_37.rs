macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl ToTokens for u128 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: u128_suffixed (* self)) ; } }
    };
}

impl_37!()