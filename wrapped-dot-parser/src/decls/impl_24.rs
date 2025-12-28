macro_rules! deps {
    () => {
        StmtList!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        # [cfg (feature = "to_tokens")] impl ToTokens for StmtList < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let stmts = & self . stmts ; let tokens = quote ! { dot_parser :: ast :: StmtList { stmts : std :: vec ! [# (# stmts) ,*] , } } ; ts . append_all (tokens) ; } }
    };
}

impl_24!();