// Generated macro for impl_396 (impl)
macro_rules! Depcrate_uri_authorityimpl_396 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_396"}
// Dependencies: {}
impl PartialOrd < Authority > for str { fn partial_cmp (& self , other : & Authority) -> Option < cmp :: Ordering > { let left = self . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; let right = other . data . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; left . partial_cmp (right) } }
};
}
