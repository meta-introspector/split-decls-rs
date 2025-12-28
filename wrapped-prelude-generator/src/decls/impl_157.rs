macro_rules! deps {
    () => {
        GeneratedTraitMethod!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl ToTokens for GeneratedTraitMethod { fn to_tokens (& self , tokens : & mut TokenStream) { let sig : TokenStream = self . signature . parse () . expect ("Invalid method signature") ; tokens . extend (quote ! { # sig }) ; } }
    };
}

impl_157!();