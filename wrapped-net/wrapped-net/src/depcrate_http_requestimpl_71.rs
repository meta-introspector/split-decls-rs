// Generated macro for impl_71 (impl)
macro_rules! Depcrate_http_requestimpl_71 {
() => {
// Module: crate::http::request
// Provides: {"impl_71"}
// Dependencies: {}
impl TryFrom < RequestBuilder > for Request { type Error = crate :: error :: Error ; fn try_from (mut value : RequestBuilder) -> Result < Self , Self :: Error > { let request = web_sys :: Request :: new_with_str (& value . url) . map_err (js_to_error) ? ; let url = web_sys :: Url :: new (& request . url ()) . map_err (js_to_error) ? ; let combined_query = match url . search () . as_str () { "" => value . query . to_string () , _ => format ! ("{}&{}" , url . search () , value . query) , } ; url . set_search (& combined_query) ; let final_url = String :: from (url . to_string ()) ; value . options . headers (& value . headers . into_raw ()) ; let request = web_sys :: Request :: new_with_str_and_init (& final_url , & value . options) . map_err (js_to_error) ? ; Ok (request . into ()) } }
};
}
