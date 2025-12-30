// Generated macro for impl_95 (impl)
macro_rules! Depcrate_http_responseimpl_95 {
() => {
// Module: crate::http::response
// Provides: {"impl_95"}
// Dependencies: {}
impl IntoRawResponse for Option < & js_sys :: Object > { fn into_raw (self , init : ResponseInit) -> Result < web_sys :: Response , JsValue > { web_sys :: Response :: new_with_opt_buffer_source_and_init (self , & init) } }
};
}
