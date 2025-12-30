// Generated macro for impl_309 (impl)
macro_rules! Depcrate_graphql_scalarimpl_309 {
() => {
// Module: crate::graphql_scalar
// Provides: {"impl_309"}
// Dependencies: {}
impl ParseToken { # [doc = " Expands [`ParseScalarValue::from_str`] method."] # [doc = ""] # [doc = " [`ParseScalarValue::from_str`]: juniper::ParseScalarValue::from_str"] fn expand_from_str (& self , scalar : & scalar :: Type) -> TokenStream { match self { Self :: Custom (parse_token) => { quote ! { # parse_token (token) } } Self :: Delegated (delegated) => delegated . iter () . fold (None , | acc , ty | { acc . map_or_else (| | Some (quote ! { <# ty as :: juniper :: ParseScalarValue <# scalar >>:: from_str (token) }) , | prev | { Some (quote ! { # prev . or_else (| _ | { <# ty as :: juniper :: ParseScalarValue <# scalar >>:: from_str (token) }) }) }) }) . unwrap_or_default () , } } }
};
}
