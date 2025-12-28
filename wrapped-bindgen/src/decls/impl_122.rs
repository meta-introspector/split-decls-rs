macro_rules! deps {
    () => {
        ToTokens!();
        TokenStream!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < T : ToTokens > ToTokens for Option < T > { fn to_tokens (& self , tokens : & mut TokenStream) { if let Some (ref t) = * self { t . to_tokens (tokens) ; } } }
    };
}

impl_122!()