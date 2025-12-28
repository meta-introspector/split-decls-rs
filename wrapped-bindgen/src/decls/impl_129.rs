macro_rules! deps {
    () => {
        ToTokens!();
        Literal!();
        TokenStream!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl ToTokens for Literal { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . push_str (self . as_str ()) ; } }
    };
}

impl_129!()