macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T : ? Sized + ToTokens > ToTokens for Box < T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
    };
}

impl_22!()