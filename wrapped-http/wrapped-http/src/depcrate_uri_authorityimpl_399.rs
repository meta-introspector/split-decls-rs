// Generated macro for impl_399 (impl)
macro_rules! Depcrate_uri_authorityimpl_399 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_399"}
// Dependencies: {}
impl PartialOrd < String > for Authority { fn partial_cmp (& self , other : & String) -> Option < cmp :: Ordering > { let left = self . data . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; let right = other . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; left . partial_cmp (right) } }
};
}
