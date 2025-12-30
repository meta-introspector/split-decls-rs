// Generated macro for impl_197 (impl)
macro_rules! Depcrate_common_access_control_allow_headersimpl_197 {
() => {
// Module: crate::common::access_control_allow_headers
// Provides: {"impl_197"}
// Dependencies: {}
impl FromIterator < HeaderName > for AccessControlAllowHeaders { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = HeaderName > , { let flat = iter . into_iter () . map (HeaderValue :: from) . collect () ; AccessControlAllowHeaders (flat) } }
};
}
