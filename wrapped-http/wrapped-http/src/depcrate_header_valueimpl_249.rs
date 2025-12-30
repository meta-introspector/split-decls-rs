// Generated macro for impl_249 (impl)
macro_rules! Depcrate_header_valueimpl_249 {
() => {
// Module: crate::header::value
// Provides: {"impl_249"}
// Dependencies: {}
impl PartialOrd < HeaderValue > for String { # [inline] fn partial_cmp (& self , other : & HeaderValue) -> Option < cmp :: Ordering > { self . as_bytes () . partial_cmp (other . as_bytes ()) } }
};
}
