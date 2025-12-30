// Generated macro for impl_247 (impl)
macro_rules! Depcrate_header_valueimpl_247 {
() => {
// Module: crate::header::value
// Provides: {"impl_247"}
// Dependencies: {}
impl PartialOrd < String > for HeaderValue { # [inline] fn partial_cmp (& self , other : & String) -> Option < cmp :: Ordering > { self . inner . partial_cmp (other . as_bytes ()) } }
};
}
