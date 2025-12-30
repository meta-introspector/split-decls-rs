// Generated macro for impl_397 (impl)
macro_rules! Depcrate_uri_authorityimpl_397 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_397"}
// Dependencies: {}
impl < 'a > PartialOrd < Authority > for & 'a str { fn partial_cmp (& self , other : & Authority) -> Option < cmp :: Ordering > { let left = self . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; let right = other . data . as_bytes () . iter () . map (| b | b . to_ascii_lowercase ()) ; left . partial_cmp (right) } }
};
}
