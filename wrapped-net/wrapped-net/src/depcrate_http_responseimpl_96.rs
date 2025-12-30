// Generated macro for impl_96 (impl)
macro_rules! Depcrate_http_responseimpl_96 {
() => {
// Module: crate::http::response
// Provides: {"impl_96"}
// Dependencies: {}
impl IntoRawResponse for Option < & mut [u8] > { fn into_raw (self , init : ResponseInit) -> Result < web_sys :: Response , JsValue > { web_sys :: Response :: new_with_opt_u8_array_and_init (self , & init) } }
};
}
