// Generated macro for impl_84 (impl)
macro_rules! Depcrate_astimpl_84 {
() => {
// Module: crate::ast
// Provides: {"impl_84"}
// Dependencies: {}
# [cfg (feature = "to_tokens")] impl ToTokens for NodeID { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let id = & self . id ; let port = match & self . port { Some (port) => quote ! { std :: option :: Option :: Some (# port) } , None => quote ! { std :: option :: Option :: None } , } ; let tokens = quote ! { dot_parser :: ast :: NodeID { id : std :: string :: ToString :: to_string (# id) , port : # port , } } ; ts . append_all (tokens) ; } }
};
}
