macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl ToTokens for TokenTree { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (self . clone ()) ; } }
    };
}

impl_49!();