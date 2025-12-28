macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl ToTokens for usize { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: usize_suffixed (* self)) ; } }
    };
}

impl_38!()