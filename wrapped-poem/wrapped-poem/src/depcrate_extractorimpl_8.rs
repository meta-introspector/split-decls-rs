// Generated macro for impl_8 (impl)
macro_rules! Depcrate_extractorimpl_8 {
() => {
// Module: crate::extractor
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'a > FromRequest < 'a > for GraphQLBatchRequest { async fn from_request (req : & 'a Request , body : & mut RequestBody) -> Result < Self > { if req . method () == Method :: GET { let req = async_graphql :: http :: parse_query_string (req . uri () . query () . unwrap_or_default ()) . map_err (BadRequest) ? ; Ok (Self (async_graphql :: BatchRequest :: Single (req))) } else { let content_type = req . headers () . get (header :: CONTENT_TYPE) . and_then (| value | value . to_str () . ok ()) . map (ToString :: to_string) ; Ok (Self (async_graphql :: http :: receive_batch_body (content_type , body . take () ? . into_async_read () . compat () , MultipartOptions :: default () ,) . await . map_err (BadRequest) ? ,)) } } }
};
}
