// Generated macro for impl_230 (impl)
macro_rules! Depcrate_common_access_control_expose_headersimpl_230 {
() => {
// Module: crate::common::access_control_expose_headers
// Provides: {"impl_230"}
// Dependencies: {}
impl FromIterator < HeaderName > for AccessControlExposeHeaders { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = HeaderName > , { let flat = iter . into_iter () . map (HeaderValue :: from) . collect () ; AccessControlExposeHeaders (flat) } }
};
}
