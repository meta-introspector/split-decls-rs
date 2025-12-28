macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T : ToTokens > ToTokens for Option < T > { fn to_tokens (& self , tokens : & mut TokenStream) { if let Some (t) = self { t . to_tokens (tokens) ; } } }
    };
}

impl_24!()