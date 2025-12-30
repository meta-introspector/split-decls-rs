// Generated macro for impl_54 (impl)
macro_rules! Depcrate_astimpl_54 {
() => {
// Module: crate::ast
// Provides: {"impl_54"}
// Dependencies: {}
# [cfg (feature = "to_tokens")] impl ToTokens for AttrList < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let elems = & self . elems ; let tokens = quote ! { dot_parser :: ast :: AttrList { elems : std :: vec ! [# (# elems) ,*] } } ; ts . append_all (tokens) ; } }
};
}
