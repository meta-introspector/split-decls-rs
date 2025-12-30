// Generated macro for parse_post_json_req (function)
macro_rules! Depcrateparse_post_json_req {
() => {
// Module: crate
// Provides: {"parse_post_json_req"}
// Dependencies: {}
async fn parse_post_json_req < S , B > (body : B ,) -> Result < GraphQLBatchRequest < S > , GraphQLRequestError < B > > where S : ScalarValue , B : Body , { let chunk = body . collect () . await . map_err (GraphQLRequestError :: BodyHyper) ? ; let input = String :: from_utf8 (chunk . to_bytes () . into ()) . map_err (GraphQLRequestError :: BodyUtf8) ? ; serde_json :: from_str :: < GraphQLBatchRequest < S > > (& input) . map_err (GraphQLRequestError :: BodyJSONError) }
};
}
