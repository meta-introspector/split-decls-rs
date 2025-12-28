macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl ToTokens for str { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: string (self)) ; } }
    };
}

impl_25!();