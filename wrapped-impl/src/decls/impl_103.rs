macro_rules! deps {
    () => {
        IdentUnraw!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl ToTokens for IdentUnraw { fn to_tokens (& self , tokens : & mut TokenStream) { self . 0 . unraw () . to_tokens (tokens) ; } }
    };
}

impl_103!()