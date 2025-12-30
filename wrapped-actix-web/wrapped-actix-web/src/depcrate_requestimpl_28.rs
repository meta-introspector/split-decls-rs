// Generated macro for impl_28 (impl)
macro_rules! Depcrate_requestimpl_28 {
() => {
// Module: crate::request
// Provides: {"impl_28"}
// Dependencies: {}
impl Responder for GraphQLResponse { type Body = BoxBody ; fn respond_to (self , req : & HttpRequest) -> HttpResponse { let mut builder = HttpResponse :: build (StatusCode :: OK) ; if self . 0 . is_ok () { if let Some (cache_control) = self . 0 . cache_control () . value () { builder . append_header ((http :: header :: CACHE_CONTROL , cache_control)) ; } } let accept = req . headers () . get (http :: header :: ACCEPT) . and_then (| val | val . to_str () . ok ()) ; let (ct , body) = match accept { # [cfg (feature = "cbor")] Some (ct @ "application/cbor") => (ct , match serde_cbor :: to_vec (& self . 0) { Ok (body) => body , Err (e) => return HttpResponse :: from_error (cbor :: Error (e)) , } ,) , _ => ("application/graphql-response+json" , match serde_json :: to_vec (& self . 0) { Ok (body) => body , Err (e) => return HttpResponse :: from_error (JsonPayloadError :: Serialize (e)) , } ,) , } ; let mut resp = builder . content_type (ct) . body (body) ; for (name , value) in self . 0 . http_headers_iter () { if let (Ok (name) , Ok (value)) = (HeaderName :: from_str (name . as_str ()) , HeaderValue :: from_bytes (value . as_bytes ()) ,) { resp . headers_mut () . append (name , value) ; } } resp } }
};
}
