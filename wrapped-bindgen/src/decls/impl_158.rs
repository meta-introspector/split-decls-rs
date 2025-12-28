macro_rules! deps {
    () => {
        RepInterp!();
        TokenStream!();
        ToTokens!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < T : ToTokens > ToTokens for RepInterp < T > { fn to_tokens (& self , tokens : & mut TokenStream) { self . 0 . to_tokens (tokens) ; } }
    };
}

impl_158!()