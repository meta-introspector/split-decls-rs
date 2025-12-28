macro_rules! deps {
    () => {
        ToTokens!();
        TokenStream!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl ToTokens for bool { fn to_tokens (& self , tokens : & mut TokenStream) { let word = if * self { "true" } else { "false" } ; tokens . push_space () ; tokens . push_str (word) ; } }
    };
}

impl_128!()