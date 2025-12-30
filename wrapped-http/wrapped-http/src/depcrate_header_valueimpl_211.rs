// Generated macro for impl_211 (impl)
macro_rules! Depcrate_header_valueimpl_211 {
() => {
// Module: crate::header::value
// Provides: {"impl_211"}
// Dependencies: {}
impl From < HeaderName > for HeaderValue { # [inline] fn from (h : HeaderName) -> HeaderValue { HeaderValue { inner : h . into_bytes () , is_sensitive : false , } } }
};
}
