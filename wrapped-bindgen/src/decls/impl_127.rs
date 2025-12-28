macro_rules! deps {
    () => {
        TokenStream!();
        ToTokens!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl ToTokens for char { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . push_space () ; tokens . push ('\'') ; tokens . push (* self) ; tokens . push ('\'') ; } }
    };
}

impl_127!()