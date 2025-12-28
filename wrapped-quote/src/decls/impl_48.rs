macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl ToTokens for Literal { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (self . clone ()) ; } }
    };
}

impl_48!();