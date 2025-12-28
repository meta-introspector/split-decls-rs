macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl ToTokens for f64 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: f64_suffixed (* self)) ; } }
    };
}

impl_40!()