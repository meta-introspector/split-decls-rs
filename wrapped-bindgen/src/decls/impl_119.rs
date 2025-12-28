macro_rules! deps {
    () => {
        ToTokens!();
        TokenStream!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < T : ? Sized + ToOwned + ToTokens > ToTokens for Cow < '_ , T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
    };
}

impl_119!();