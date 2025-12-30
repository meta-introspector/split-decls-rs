// Generated macro for impl_72 (impl)
macro_rules! Depcrate_astimpl_72 {
() => {
// Module: crate::ast
// Provides: {"impl_72"}
// Dependencies: {}
# [cfg (feature = "to_tokens")] impl ToTokens for EdgeStmt < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let from = match & self . from { Either :: Left (id) => { quote ! { dot_parser :: ast :: either :: Either :: Left (# id) } } Either :: Right (sub) => { quote ! { dot_parser :: ast :: either :: Either :: Right (# sub) } } } ; let attr = match & self . attr { Some (attr) => { quote ! { std :: option :: Option :: Some (# attr) } } None => { quote ! { std :: option :: Option :: None } } } ; let next = & self . next ; let tokens = quote ! { dot_parser :: ast :: EdgeStmt { from : # from , next : # next , attr : # attr , } } ; ts . append_all (tokens) ; } }
};
}
