macro_rules! deps {
    () => {
        Deprecation!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl ToTokens for Deprecation { fn to_tokens (& self , ts : & mut TokenStream) { let tokens = if cfg ! (feature = "deprecated") { let Deprecation { span , id , version , description , } = self ; let span = * span ; let id = Ident :: new (id , span) ; quote_spanned ! (span => { # [deprecated (since = # version , note = # description)] fn # id () { } # id () ; }) } else { quote ! () } ; tokens . to_tokens (ts) ; } }
    };
}

impl_68!();