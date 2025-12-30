// Generated macro for impl_20 (impl)
macro_rules! Depcrate_requestimpl_20 {
() => {
// Module: crate::request
// Provides: {"impl_20"}
// Dependencies: {}
impl FromRequest for GraphQLRequest { type Error = Error ; type Future = future :: Map < < GraphQLBatchRequest as FromRequest > :: Future , BatchToRequestMapper > ; fn from_request (req : & HttpRequest , payload : & mut Payload) -> Self :: Future { GraphQLBatchRequest :: from_request (req , payload) . map (| res | { Ok (Self (res ? . 0 . into_single () . map_err (actix_web :: error :: ErrorBadRequest) ? ,)) }) } }
};
}
