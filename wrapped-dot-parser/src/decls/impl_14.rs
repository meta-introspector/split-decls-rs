macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (feature = "to_tokens")] impl ToTokens for Graph < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let name = match & self . name { Some (name) => { quote ! { std :: option :: Option :: Some (std :: string :: ToString :: to_string (# name)) } } None => quote ! { std :: option :: Option :: None } , } ; let strict = self . strict ; let is_digraph = self . is_digraph ; let stmts = & self . stmts ; let tokens = quote ! { dot_parser :: ast :: Graph ::< (&'static str , &'static str) > { strict : # strict , is_digraph : # is_digraph , name : # name , stmts : # stmts } } ; ts . append_all (tokens) ; } }
    };
}

impl_14!()