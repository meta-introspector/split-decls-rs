// Generated macro for impl_100 (impl)
macro_rules! Depcrate_http_responseimpl_100 {
() => {
// Module: crate::http::response
// Provides: {"impl_100"}
// Dependencies: {}
impl IntoRawResponse for Option < & web_sys :: ReadableStream > { fn into_raw (self , init : ResponseInit) -> Result < web_sys :: Response , JsValue > { web_sys :: Response :: new_with_opt_readable_stream_and_init (self , & init) } }
};
}
