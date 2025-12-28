macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl ToTokens for CStr { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: c_string (self)) ; } }
    };
}

impl_43!()