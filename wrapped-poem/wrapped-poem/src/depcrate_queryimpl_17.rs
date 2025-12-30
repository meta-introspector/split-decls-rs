// Generated macro for impl_17 (impl)
macro_rules! Depcrate_queryimpl_17 {
() => {
// Module: crate::query
// Provides: {"impl_17"}
// Dependencies: {}
impl < E > Endpoint for GraphQL < E > where E : Executor , { type Output = Response ; async fn call (& self , req : Request) -> Result < Self :: Output > { let is_accept_multipart_mixed = req . header ("accept") . map (is_accept_multipart_mixed) . unwrap_or_default () ; if is_accept_multipart_mixed { let (req , mut body) = req . split () ; let req = GraphQLRequest :: from_request (& req , & mut body) . await ? ; let stream = self . executor . execute_stream (req . 0 , None) ; Ok (Response :: builder () . header ("content-type" , "multipart/mixed; boundary=graphql") . body (Body :: from_bytes_stream (create_multipart_mixed_stream (stream , Duration :: from_secs (30)) . map (Ok :: < _ , std :: io :: Error >) ,))) } else { let (req , mut body) = req . split () ; let req = GraphQLBatchRequest :: from_request (& req , & mut body) . await ? ; Ok (GraphQLBatchResponse (self . executor . execute_batch (req . 0) . await) . into_response ()) } } }
};
}
