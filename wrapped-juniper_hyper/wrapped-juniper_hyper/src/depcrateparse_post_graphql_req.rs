// Generated macro for parse_post_graphql_req (function)
macro_rules! Depcrateparse_post_graphql_req {
() => {
// Module: crate
// Provides: {"parse_post_graphql_req"}
// Dependencies: {}
async fn parse_post_graphql_req < S , B > (body : B ,) -> Result < GraphQLBatchRequest < S > , GraphQLRequestError < B > > where S : ScalarValue , B : Body , { let chunk = body . collect () . await . map_err (GraphQLRequestError :: BodyHyper) ? ; let query = String :: from_utf8 (chunk . to_bytes () . into ()) . map_err (GraphQLRequestError :: BodyUtf8) ? ; Ok (GraphQLBatchRequest :: Single (GraphQLRequest :: new (query , None , None ,))) }
};
}
