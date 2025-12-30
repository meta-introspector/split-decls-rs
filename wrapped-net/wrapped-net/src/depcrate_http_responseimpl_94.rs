// Generated macro for impl_94 (impl)
macro_rules! Depcrate_http_responseimpl_94 {
() => {
// Module: crate::http::response
// Provides: {"impl_94"}
// Dependencies: {}
impl IntoRawResponse for Option < & web_sys :: Blob > { fn into_raw (self , init : ResponseInit) -> Result < web_sys :: Response , JsValue > { web_sys :: Response :: new_with_opt_blob_and_init (self , & init) } }
};
}
