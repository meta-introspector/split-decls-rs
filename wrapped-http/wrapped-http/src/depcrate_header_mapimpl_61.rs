// Generated macro for impl_61 (impl)
macro_rules! Depcrate_header_mapimpl_61 {
() => {
// Module: crate::header::map
// Provides: {"impl_61"}
// Dependencies: {}
# [doc = " Try to convert a `HashMap` into a `HeaderMap`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = " use std::convert::TryInto;"] # [doc = " use http::HeaderMap;"] # [doc = ""] # [doc = " let mut map = HashMap::new();"] # [doc = " map.insert(\"X-Custom-Header\".to_string(), \"my value\".to_string());"] # [doc = ""] # [doc = " let headers: HeaderMap = (&map).try_into().expect(\"valid headers\");"] # [doc = " assert_eq!(headers[\"X-Custom-Header\"], \"my value\");"] # [doc = " ```"] impl < 'a , K , V , S , T > TryFrom < & 'a HashMap < K , V , S > > for HeaderMap < T > where K : Eq + Hash , HeaderName : TryFrom < & 'a K > , < HeaderName as TryFrom < & 'a K > > :: Error : Into < crate :: Error > , T : TryFrom < & 'a V > , T :: Error : Into < crate :: Error > , { type Error = Error ; fn try_from (c : & 'a HashMap < K , V , S >) -> Result < Self , Self :: Error > { c . iter () . map (| (k , v) | -> crate :: Result < (HeaderName , T) > { let name = TryFrom :: try_from (k) . map_err (Into :: into) ? ; let value = TryFrom :: try_from (v) . map_err (Into :: into) ? ; Ok ((name , value)) }) . collect () } }
};
}
