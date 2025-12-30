// Generated macro for impl_53 (impl)
macro_rules! Depcrate_http_queryimpl_53 {
() => {
// Module: crate::http::query
// Provides: {"impl_53"}
// Dependencies: {}
# [allow (dead_code)] impl QueryParams { # [doc = " Create a new empty query parameters object."] pub fn new () -> Self { Self { raw : web_sys :: UrlSearchParams :: new () . unwrap_throw () , } } # [doc = " Create [`QueryParams`] from [`web_sys::UrlSearchParams`] object."] pub fn from_raw (raw : web_sys :: UrlSearchParams) -> Self { Self { raw } } # [doc = " Append a parameter to the query string."] pub fn append (& self , name : & str , value : & str) { self . raw . append (name , value) } # [doc = " Get the value of a parameter. If the parameter has multiple occurrences, the first value is"] # [doc = " returned."] pub fn get (& self , name : & str) -> Option < String > { self . raw . get (name) } # [doc = " Get all associated values of a parameter."] pub fn get_all (& self , name : & str) -> Vec < String > { self . raw . get_all (name) . iter () . map (| jsval | jsval . as_string () . unwrap_throw ()) . collect () } # [doc = " Remove all occurrences of a parameter from the query string."] pub fn delete (& self , name : & str) { self . raw . delete (name) } # [doc = " Iterate over (name, value) pairs of the query parameters."] pub fn iter (& self) -> impl Iterator < Item = (String , String) > { let fake_map : & Map = self . raw . unchecked_ref () ; UncheckedIter :: from (fake_map . entries ()) . map (| entry | { let entry : Array = entry . unchecked_into () ; let key = entry . get (0) ; let value = entry . get (1) ; (key . as_string () . unwrap_throw () , value . as_string () . unwrap_throw () ,) }) } }
};
}
