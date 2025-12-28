macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl ToTokens for i8 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: i8_suffixed (* self)) ; } }
    };
}

impl_27!()