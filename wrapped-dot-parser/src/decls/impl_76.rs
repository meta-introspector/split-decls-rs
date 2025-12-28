macro_rules! deps {
    () => {
        Subgraph!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        # [cfg (feature = "to_tokens")] impl ToTokens for Subgraph < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let id = match & self . id { Some (s) => quote ! { std :: option :: Option :: Some (# s) } , None => quote ! { std :: option :: Option :: None } , } ; let stmts = & self . stmts ; let tokens = quote ! { dot_parser :: ast :: Subgraph { id : # id , stmts : # stmts , } } ; ts . append_all (tokens) ; } }
    };
}

impl_76!();