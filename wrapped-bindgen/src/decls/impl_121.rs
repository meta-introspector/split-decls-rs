macro_rules! deps {
    () => {
        TokenStream!();
        ToTokens!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < T : ? Sized + ToTokens > ToTokens for Rc < T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
    };
}

impl_121!()