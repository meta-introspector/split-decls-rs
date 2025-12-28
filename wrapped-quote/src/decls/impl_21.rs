macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a , T : ? Sized + ToOwned + ToTokens > ToTokens for Cow < 'a , T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
    };
}

impl_21!();