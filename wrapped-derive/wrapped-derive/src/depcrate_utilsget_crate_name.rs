// Generated macro for get_crate_name (function)
macro_rules! Depcrate_utilsget_crate_name {
() => {
// Module: crate::utils
// Provides: {"get_crate_name"}
// Dependencies: {}
pub fn get_crate_name (internal : bool) -> TokenStream { if internal { quote ! { crate } } else { let name = match crate_name ("async-graphql") { Ok (FoundCrate :: Name (name)) => name , Ok (FoundCrate :: Itself) | Err (_) => "async_graphql" . to_string () , } ; TokenTree :: from (Ident :: new (& name , Span :: call_site ())) . into () } }
};
}
