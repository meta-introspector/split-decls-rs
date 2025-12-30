// Generated macro for impl_39 (impl)
macro_rules! Depcrate_astimpl_39 {
() => {
// Module: crate::ast
// Provides: {"impl_39"}
// Dependencies: {}
# [cfg (feature = "to_tokens")] impl ToTokens for StmtList < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let stmts = & self . stmts ; let tokens = quote ! { dot_parser :: ast :: StmtList { stmts : std :: vec ! [# (# stmts) ,*] , } } ; ts . append_all (tokens) ; } }
};
}
