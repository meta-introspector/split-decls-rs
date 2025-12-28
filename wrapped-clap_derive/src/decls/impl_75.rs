macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl ToTokens for Name { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Name :: Assigned (t) => t . to_tokens (tokens) , Name :: Derived (ident) => { let s = ident . unraw () . to_string () ; quote_spanned ! (ident . span () => # s) . to_tokens (tokens) ; } } } }
    };
}

impl_75!()