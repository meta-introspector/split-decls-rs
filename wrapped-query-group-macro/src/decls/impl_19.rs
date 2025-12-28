macro_rules! deps {
    () => {
        TrackedQuery!();
        InputQuery!();
        Transparent!();
        Intern!();
        Queries!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl ToTokens for Queries { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { match self { Queries :: TrackedQuery (tracked_query) => tracked_query . to_tokens (tokens) , Queries :: InputQuery (input_query) => input_query . to_tokens (tokens) , Queries :: Transparent (transparent) => transparent . to_tokens (tokens) , Queries :: Intern (intern) => intern . to_tokens (tokens) , } } }
    };
}

impl_19!()