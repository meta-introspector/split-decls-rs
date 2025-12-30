// Generated macro for parse_get_req (function)
macro_rules! Depcrateparse_get_req {
() => {
// Module: crate
// Provides: {"parse_get_req"}
// Dependencies: {}
fn parse_get_req < S , B > (req : Request < B >) -> Result < GraphQLBatchRequest < S > , GraphQLRequestError < B > > where S : ScalarValue , B : Body , { req . uri () . query () . map (| q | gql_request_from_get (q) . map (GraphQLBatchRequest :: Single)) . unwrap_or_else (| | { Err (GraphQLRequestError :: Invalid ("'query' parameter is missing" . into () ,)) }) }
};
}
