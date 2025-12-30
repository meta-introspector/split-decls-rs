// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl From < GraphQLQuery > for GraphQLRequest { fn from (query : GraphQLQuery) -> Self { let mut request = async_graphql :: Request :: new (query . query) ; if let Some (operation_name) = query . operation_name { request = request . operation_name (operation_name) ; } if let Some (variables) = query . variables { let value = serde_json :: from_str (& variables) . unwrap_or_default () ; let variables = async_graphql :: Variables :: from_json (value) ; request = request . variables (variables) ; } GraphQLRequest (request) } }
};
}
