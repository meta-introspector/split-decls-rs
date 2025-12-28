macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl ToTokens for f32 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: f32_suffixed (* self)) ; } }
    };
}

impl_39!();