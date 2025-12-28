macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl ToTokens for Punct { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (self . clone ()) ; } }
    };
}

impl_47!();