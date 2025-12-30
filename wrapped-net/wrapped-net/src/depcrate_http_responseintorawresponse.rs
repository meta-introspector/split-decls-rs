// Generated macro for IntoRawResponse (trait)
macro_rules! Depcrate_http_responseIntoRawResponse {
() => {
// Module: crate::http::response
// Provides: {"IntoRawResponse"}
// Dependencies: {}
# [doc = " trait which allow consuming self into a raw web_sys::Response"] pub trait IntoRawResponse { # [doc = " A method which converts `self` and a [`web_sys::ResponseInit`] into a result to a"] # [doc = " [`web_sys::Response`]."] fn into_raw (self , init : ResponseInit) -> Result < web_sys :: Response , JsValue > ; }
};
}
