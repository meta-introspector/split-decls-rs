macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T : ? Sized + ToTokens > ToTokens for & mut T { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
    };
}

impl_20!();