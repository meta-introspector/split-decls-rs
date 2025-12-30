// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl < S > From < GetGraphQLRequest > for GraphQLRequest < S > where S : ScalarValue , { fn from (get_req : GetGraphQLRequest) -> Self { let GetGraphQLRequest { query , operation_name , variables , } = get_req ; let variables = variables . map (| s | serde_json :: from_str (& s) . unwrap ()) ; Self :: new (query , operation_name , variables) } }
};
}
