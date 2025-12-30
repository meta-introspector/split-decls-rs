// Generated macro for impl_6 (impl)
macro_rules! Depcrate_extractorimpl_6 {
() => {
// Module: crate::extractor
// Provides: {"impl_6"}
// Dependencies: {}
impl < 'a > FromRequest < 'a > for GraphQLRequest { async fn from_request (req : & 'a Request , body : & mut RequestBody) -> Result < Self > { Ok (GraphQLRequest (GraphQLBatchRequest :: from_request (req , body) . await ? . 0 . into_single () . map_err (BadRequest) ? ,)) } }
};
}
