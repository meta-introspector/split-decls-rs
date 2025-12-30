// Generated macro for impl_87 (impl)
macro_rules! Depcrate_astimpl_87 {
() => {
// Module: crate::ast
// Provides: {"impl_87"}
// Dependencies: {}
# [cfg (feature = "to_tokens")] impl ToTokens for Port { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let tokens = match self { Port :: ID (s , cpss) => match cpss { Some (cpss) => { quote ! { dot_parser :: ast :: Port :: ID (std :: string :: ToString :: to_string (# s) , std :: option :: Option :: Some (# cpss)) } } None => { quote ! { dot_parser :: ast :: Port :: ID (std :: string :: ToString :: to_string (# s) , std :: option :: Option :: None) } } } , Port :: Compass (cpss) => { quote ! { dot_parser :: ast :: Port :: Compass (# cpss) } } } ; ts . append_all (tokens) ; } }
};
}
