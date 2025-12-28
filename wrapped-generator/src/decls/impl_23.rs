macro_rules! deps {
    () => {
        QuoteOption!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < T : ToTokens > ToTokens for QuoteOption < T > { fn to_tokens (& self , tokens : & mut TokenStream) { let option = option_type () ; tokens . append_all (match self . 0 { Some (ref t) => quote ! { # option :: Some (# t) } , None => quote ! { # option :: None } , }) ; } }
    };
}

impl_23!();