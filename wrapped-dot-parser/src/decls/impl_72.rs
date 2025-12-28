macro_rules! deps {
    () => {
        ID!();
        Port!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        # [cfg (feature = "to_tokens")] impl ToTokens for Port { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let tokens = match self { Port :: ID (s , cpss) => match cpss { Some (cpss) => { quote ! { dot_parser :: ast :: Port :: ID (std :: string :: ToString :: to_string (# s) , std :: option :: Option :: Some (# cpss)) } } None => { quote ! { dot_parser :: ast :: Port :: ID (std :: string :: ToString :: to_string (# s) , std :: option :: Option :: None) } } } , Port :: Compass (cpss) => { quote ! { dot_parser :: ast :: Port :: Compass (# cpss) } } } ; ts . append_all (tokens) ; } }
    };
}

impl_72!()