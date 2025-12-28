macro_rules! deps {
    () => {
        EdgeRHS!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        # [cfg (feature = "to_tokens")] impl ToTokens for EdgeRHS < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let to = match & self . to { Either :: Left (id) => quote ! { dot_parser :: ast :: either :: Either :: Left (# id) } , Either :: Right (sub) => quote ! { dot_parser :: ast :: either :: Either :: Right (# sub) } , } ; let next = match & self . next { Some (next) => quote ! { std :: option :: Option :: Some (std :: boxed :: Box :: new (# next)) } , None => quote ! { std :: option :: Option :: None } , } ; let tokens = quote ! { dot_parser :: ast :: EdgeRHS { to : # to , next : # next , } } ; ts . append_all (tokens) ; } }
    };
}

impl_61!();