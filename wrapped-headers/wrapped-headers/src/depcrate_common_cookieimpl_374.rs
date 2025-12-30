// Generated macro for impl_374 (impl)
macro_rules! Depcrate_common_cookieimpl_374 {
() => {
// Module: crate::common::cookie
// Provides: {"impl_374"}
// Dependencies: {}
impl Cookie { # [doc = " Lookup a value for a cookie name."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::{Cookie, HeaderMap, HeaderMapExt, HeaderValue};"] # [doc = ""] # [doc = " // Setup the header map with strings..."] # [doc = " let mut headers = HeaderMap::new();"] # [doc = " headers.insert(\"cookie\", HeaderValue::from_static(\"lang=en-US\"));"] # [doc = ""] # [doc = " // Parse a `Cookie` so we can play with it..."] # [doc = " let cookie = headers"] # [doc = "     .typed_get::<Cookie>()"] # [doc = "     .expect(\"we just inserted a valid Cookie\");"] # [doc = ""] # [doc = " assert_eq!(cookie.get(\"lang\"), Some(\"en-US\"));"] # [doc = " assert_eq!(cookie.get(\"SID\"), None);"] # [doc = " ```"] pub fn get (& self , name : & str) -> Option < & str > { self . iter () . find (| & (key , _) | key == name) . map (| (_ , val) | val) } # [doc = " Get the number of key-value pairs this `Cookie` contains."] pub fn len (& self) -> usize { self . iter () . count () } # [doc = " Iterator the key-value pairs of this `Cookie` header."] pub fn iter (& self) -> impl Iterator < Item = (& str , & str) > { self . 0 . iter () . filter_map (| kv | { let mut iter = kv . splitn (2 , '=') ; let key = iter . next () ? . trim () ; let val = iter . next () ? . trim () ; Some ((key , val)) }) } }
};
}
