// Generated macro for impl_99 (impl)
macro_rules! Depcrate_http_responseimpl_99 {
() => {
// Module: crate::http::response
// Provides: {"impl_99"}
// Dependencies: {}
impl IntoRawResponse for Option < & str > { fn into_raw (self , init : ResponseInit) -> Result < web_sys :: Response , JsValue > { web_sys :: Response :: new_with_opt_str_and_init (self , & init) } }
};
}
