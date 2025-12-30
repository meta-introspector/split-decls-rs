// Generated macro for impl_80 (impl)
macro_rules! Depcrate_astimpl_80 {
() => {
// Module: crate::ast
// Provides: {"impl_80"}
// Dependencies: {}
# [cfg (feature = "to_tokens")] impl ToTokens for NodeStmt < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let node = & self . node ; let attrs = match & self . attr { Some (a) => quote ! { std :: option :: Option :: Some (# a) } , None => quote ! { std :: option :: Option :: None } , } ; let tokens = quote ! { dot_parser :: ast :: NodeStmt { node : # node , attr : # attrs } } ; ts . append_all (tokens) ; } }
};
}
