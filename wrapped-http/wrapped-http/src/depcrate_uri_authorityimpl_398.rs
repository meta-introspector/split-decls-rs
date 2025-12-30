// Generated macro for impl_398 (impl)
macro_rules! Depcrate_uri_authorityimpl_398 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_398"}
// Dependencies: {}
impl < 'a > PartialOrd < & 'a str > for Authority { fn partial_cmp (& self , other : & & 'a str) -> Option < cmp :: Ordering > { let left = self . data . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; let right = other . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; left . partial_cmp (right) } }
};
}
