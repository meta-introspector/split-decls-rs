// Generated macro for impl_91 (impl)
macro_rules! Depcrate_astimpl_91 {
() => {
// Module: crate::ast
// Provides: {"impl_91"}
// Dependencies: {}
# [cfg (feature = "to_tokens")] impl ToTokens for Subgraph < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let id = match & self . id { Some (s) => quote ! { std :: option :: Option :: Some (# s) } , None => quote ! { std :: option :: Option :: None } , } ; let stmts = & self . stmts ; let tokens = quote ! { dot_parser :: ast :: Subgraph { id : # id , stmts : # stmts , } } ; ts . append_all (tokens) ; } }
};
}
