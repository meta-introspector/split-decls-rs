// Generated macro for impl_240 (impl)
macro_rules! Depcrate_header_valueimpl_240 {
() => {
// Module: crate::header::value
// Provides: {"impl_240"}
// Dependencies: {}
impl PartialOrd < str > for HeaderValue { # [inline] fn partial_cmp (& self , other : & str) -> Option < cmp :: Ordering > { (* self . inner) . partial_cmp (other . as_bytes ()) } }
};
}
