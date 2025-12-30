// Generated macro for impl_98 (impl)
macro_rules! Depcrate_http_responseimpl_98 {
() => {
// Module: crate::http::response
// Provides: {"impl_98"}
// Dependencies: {}
impl IntoRawResponse for Option < & web_sys :: UrlSearchParams > { fn into_raw (self , init : ResponseInit) -> Result < web_sys :: Response , JsValue > { web_sys :: Response :: new_with_opt_url_search_params_and_init (self , & init) } }
};
}
