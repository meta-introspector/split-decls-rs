// Generated macro for impl_395 (impl)
macro_rules! Depcrate_uri_authorityimpl_395 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_395"}
// Dependencies: {}
impl PartialOrd < str > for Authority { fn partial_cmp (& self , other : & str) -> Option < cmp :: Ordering > { let left = self . data . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; let right = other . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; left . partial_cmp (right) } }
};
}
