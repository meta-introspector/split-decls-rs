macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl ToTokens for Ident { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (self . clone ()) ; } }
    };
}

impl_46!()