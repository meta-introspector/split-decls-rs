macro_rules! deps {
    () => {
        Sp!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < T : ToTokens > ToTokens for Sp < T > { fn to_tokens (& self , stream : & mut TokenStream) { let tt = self . val . to_token_stream () . into_iter () . map (| mut tt | { tt . set_span (self . span) ; tt }) ; stream . extend (tt) ; } }
    };
}

impl_100!();