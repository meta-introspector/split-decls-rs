macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl ToTokens for char { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: character (* self)) ; } }
    };
}

impl_41!()