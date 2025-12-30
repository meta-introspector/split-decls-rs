// Generated macro for impl_255 (impl)
macro_rules! Depcrate_header_valueimpl_255 {
() => {
// Module: crate::header::value
// Provides: {"impl_255"}
// Dependencies: {}
impl < 'a > PartialOrd < HeaderValue > for & 'a str { # [inline] fn partial_cmp (& self , other : & HeaderValue) -> Option < cmp :: Ordering > { self . as_bytes () . partial_cmp (other . as_bytes ()) } }
};
}
