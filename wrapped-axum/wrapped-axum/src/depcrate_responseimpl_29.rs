// Generated macro for impl_29 (impl)
macro_rules! Depcrate_responseimpl_29 {
() => {
// Module: crate::response
// Provides: {"impl_29"}
// Dependencies: {}
impl IntoResponse for GraphQLResponse { fn into_response (self) -> Response { let body : Body = serde_json :: to_string (& self . 0) . unwrap () . into () ; let mut resp = Response :: new (body) ; resp . headers_mut () . insert (http :: header :: CONTENT_TYPE , HeaderValue :: from_static ("application/graphql-response+json") ,) ; if self . 0 . is_ok () { if let Some (cache_control) = self . 0 . cache_control () . value () { if let Ok (value) = HeaderValue :: from_str (& cache_control) { resp . headers_mut () . insert (http :: header :: CACHE_CONTROL , value) ; } } } resp . headers_mut () . extend (self . 0 . http_headers ()) ; resp } }
};
}
