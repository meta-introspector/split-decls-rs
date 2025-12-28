macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl ToTokens for isize { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: isize_suffixed (* self)) ; } }
    };
}

impl_32!()