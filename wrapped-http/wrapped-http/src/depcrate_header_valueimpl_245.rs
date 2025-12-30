// Generated macro for impl_245 (impl)
macro_rules! Depcrate_header_valueimpl_245 {
() => {
// Module: crate::header::value
// Provides: {"impl_245"}
// Dependencies: {}
impl PartialOrd < HeaderValue > for [u8] { # [inline] fn partial_cmp (& self , other : & HeaderValue) -> Option < cmp :: Ordering > { self . partial_cmp (other . as_bytes ()) } }
};
}
