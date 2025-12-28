macro_rules! deps {
    () => {
        ToTokens!();
        TokenStream!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl ToTokens for str { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . push_str (" \"") ; tokens . push_str (self) ; tokens . push ('"') ; } }
    };
}

impl_123!();