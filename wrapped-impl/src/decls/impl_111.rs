macro_rules! deps {
    () => {
        MemberUnraw!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl ToTokens for MemberUnraw { fn to_tokens (& self , tokens : & mut TokenStream) { match self { MemberUnraw :: Named (ident) => ident . to_local () . to_tokens (tokens) , MemberUnraw :: Unnamed (index) => index . to_tokens (tokens) , } } }
    };
}

impl_111!();