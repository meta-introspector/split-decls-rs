// Generated macro for parse_req (function)
macro_rules! Depcrateparse_req {
() => {
// Module: crate
// Provides: {"parse_req"}
// Dependencies: {}
async fn parse_req < S , B > (req : Request < B >) -> Result < GraphQLBatchRequest < S > , Response < String > > where S : ScalarValue , B : Body < Error : Display > , { match * req . method () { Method :: GET => parse_get_req (req) , Method :: POST => { let content_type = req . headers () . get (header :: CONTENT_TYPE) . map (HeaderValue :: to_str) ; match content_type { Some (Ok ("application/json")) => parse_post_json_req (req . into_body ()) . await , Some (Ok ("application/graphql")) => parse_post_graphql_req (req . into_body ()) . await , _ => return Err (new_response (StatusCode :: BAD_REQUEST)) , } } _ => return Err (new_response (StatusCode :: METHOD_NOT_ALLOWED)) , } . map_err (render_error) }
};
}
