// Generated macro for impl_394 (impl)
macro_rules! Depcrate_uri_authorityimpl_394 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_394"}
// Dependencies: {}
# [doc = " Case-insensitive ordering"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use http::uri::Authority;"] # [doc = " let authority: Authority = \"DEF.com\".parse().unwrap();"] # [doc = " assert!(authority < \"ghi.com\");"] # [doc = " assert!(authority > \"abc.com\");"] # [doc = " ```"] impl PartialOrd for Authority { fn partial_cmp (& self , other : & Authority) -> Option < cmp :: Ordering > { let left = self . data . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; let right = other . data . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; left . partial_cmp (right) } }
};
}
