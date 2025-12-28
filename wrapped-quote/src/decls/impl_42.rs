macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl ToTokens for bool { fn to_tokens (& self , tokens : & mut TokenStream) { let word = if * self { "true" } else { "false" } ; tokens . append (Ident :: new (word , Span :: call_site ())) ; } }
    };
}

impl_42!()