macro_rules! deps {
    () => {
        EdgeStmt!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        # [cfg (feature = "to_tokens")] impl ToTokens for EdgeStmt < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let from = match & self . from { Either :: Left (id) => { quote ! { dot_parser :: ast :: either :: Either :: Left (# id) } } Either :: Right (sub) => { quote ! { dot_parser :: ast :: either :: Either :: Right (# sub) } } } ; let attr = match & self . attr { Some (attr) => { quote ! { std :: option :: Option :: Some (# attr) } } None => { quote ! { std :: option :: Option :: None } } } ; let next = & self . next ; let tokens = quote ! { dot_parser :: ast :: EdgeStmt { from : # from , next : # next , attr : # attr , } } ; ts . append_all (tokens) ; } }
    };
}

impl_57!()