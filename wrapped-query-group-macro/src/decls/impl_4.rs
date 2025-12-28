macro_rules! deps {
    () => {
        InputQuery!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl ToTokens for InputQuery { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let sig = & self . signature ; let fn_ident = & sig . ident ; let create_data_ident = & self . create_data_ident ; let method = quote ! { # sig { let data = # create_data_ident (self) ; data .# fn_ident (self) . unwrap () } } ; method . to_tokens (tokens) ; } }
    };
}

impl_4!()