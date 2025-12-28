macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl ToTokens for i128 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: i128_suffixed (* self)) ; } }
    };
}

impl_31!()