// Generated macro for impl_97 (impl)
macro_rules! Depcrate_http_responseimpl_97 {
() => {
// Module: crate::http::response
// Provides: {"impl_97"}
// Dependencies: {}
impl IntoRawResponse for Option < & web_sys :: FormData > { fn into_raw (self , init : ResponseInit) -> Result < web_sys :: Response , JsValue > { web_sys :: Response :: new_with_opt_form_data_and_init (self , & init) } }
};
}
