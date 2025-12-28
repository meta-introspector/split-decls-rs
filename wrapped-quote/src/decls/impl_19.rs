macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < T : ? Sized + ToTokens > ToTokens for & T { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
    };
}

impl_19!();