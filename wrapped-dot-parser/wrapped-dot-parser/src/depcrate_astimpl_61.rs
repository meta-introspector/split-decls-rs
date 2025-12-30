// Generated macro for impl_61 (impl)
macro_rules! Depcrate_astimpl_61 {
() => {
// Module: crate::ast
// Provides: {"impl_61"}
// Dependencies: {}
# [cfg (feature = "to_tokens")] impl ToTokens for AList < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let elems = & self . elems ; let elems = elems . iter () . map (| (s1 , s2) | quote ! { (# s1 , # s2) }) ; let tokens = quote ! { dot_parser :: ast :: AList { elems : std :: vec ! [# (# elems) ,*] } } ; ts . append_all (tokens) ; } }
};
}
