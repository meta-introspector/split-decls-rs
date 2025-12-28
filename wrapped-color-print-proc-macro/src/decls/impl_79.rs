macro_rules! deps {
    () => {
        FormatArg!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl ToTokens for FormatArg { fn to_tokens (& self , tokens : & mut TokenStream2) { if let Some ((arg_name , eq)) = & self . arg_name { arg_name . to_tokens (tokens) ; eq . to_tokens (tokens) ; } self . expr . to_tokens (tokens) ; } }
    };
}

impl_79!();