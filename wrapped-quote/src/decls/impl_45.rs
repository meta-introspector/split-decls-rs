macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl ToTokens for Group { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (self . clone ()) ; } }
    };
}

impl_45!()