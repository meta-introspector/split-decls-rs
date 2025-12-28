macro_rules! deps {
    () => {
        ToTokens!();
        DeriveWriter!();
        TokenStream!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl ToTokens for DeriveWriter { fn to_tokens (& self , tokens : & mut TokenStream) { if ! self . 0 . is_empty () { let derive = self . 0 . iter () . map (| derive | to_ident (derive)) ; tokens . combine (quote ! { # [derive (# (# derive) ,*)] }) } } }
    };
}

impl_26!()