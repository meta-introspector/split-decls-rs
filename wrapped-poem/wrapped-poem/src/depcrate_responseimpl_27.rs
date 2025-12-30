// Generated macro for impl_27 (impl)
macro_rules! Depcrate_responseimpl_27 {
() => {
// Module: crate::response
// Provides: {"impl_27"}
// Dependencies: {}
impl IntoResponse for GraphQLBatchResponse { fn into_response (self) -> Response { let mut resp = Json (& self . 0) . into_response () ; if self . 0 . is_ok () { if let Some (cache_control) = self . 0 . cache_control () . value () { if let Ok (value) = cache_control . try_into () { resp . headers_mut () . insert ("cache-control" , value) ; } } } resp . headers_mut () . extend (self . 0 . http_headers () . iter () . filter_map (| (name , value) | { HeaderName :: from_str (name . as_str ()) . ok () . zip (HeaderValue :: from_bytes (value . as_bytes ()) . ok ()) })) ; resp } }
};
}
