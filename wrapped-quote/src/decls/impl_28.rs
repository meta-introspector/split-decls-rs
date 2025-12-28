macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl ToTokens for i16 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: i16_suffixed (* self)) ; } }
    };
}

impl_28!();