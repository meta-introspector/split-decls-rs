// Generated macro for impl_400 (impl)
macro_rules! Depcrate_uri_authorityimpl_400 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_400"}
// Dependencies: {}
impl PartialOrd < Authority > for String { fn partial_cmp (& self , other : & Authority) -> Option < cmp :: Ordering > { let left = self . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; let right = other . data . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; left . partial_cmp (right) } }
};
}
