macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl ToTokens for CString { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: c_string (self)) ; } }
    };
}

impl_44!()