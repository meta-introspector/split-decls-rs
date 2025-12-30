// Generated macro for impl_46 (impl)
macro_rules! Depcrate_astimpl_46 {
() => {
// Module: crate::ast
// Provides: {"impl_46"}
// Dependencies: {}
# [cfg (feature = "to_tokens")] impl ToTokens for Stmt < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let tokens = match & self { Self :: NodeStmt (stmt) => { quote ! { dot_parser :: ast :: Stmt :: NodeStmt (# stmt) } } Self :: EdgeStmt (stmt) => { quote ! { dot_parser :: ast :: Stmt :: EdgeStmt (# stmt) } } Self :: AttrStmt (stmt) => { quote ! { dot_parser :: ast :: Stmt :: AttrStmt (# stmt) } } Self :: IDEq (s1 , s2) => { quote ! { dot_parser :: ast :: Stmt :: IDEq (std :: string :: ToString :: to_string (# s1) , std :: string :: ToString :: to_string (# s2)) } } Self :: Subgraph (sub) => { quote ! { dot_parser :: ast :: Stmt :: Subgraph (# sub) } } } ; ts . append_all (tokens) ; } }
};
}
