// Generated macro for impl_244 (impl)
macro_rules! Depcrate_header_valueimpl_244 {
() => {
// Module: crate::header::value
// Provides: {"impl_244"}
// Dependencies: {}
impl PartialOrd < HeaderValue > for str { # [inline] fn partial_cmp (& self , other : & HeaderValue) -> Option < cmp :: Ordering > { self . as_bytes () . partial_cmp (other . as_bytes ()) } }
};
}
