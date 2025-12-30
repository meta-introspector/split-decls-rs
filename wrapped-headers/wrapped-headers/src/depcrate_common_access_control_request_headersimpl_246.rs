// Generated macro for impl_246 (impl)
macro_rules! Depcrate_common_access_control_request_headersimpl_246 {
() => {
// Module: crate::common::access_control_request_headers
// Provides: {"impl_246"}
// Dependencies: {}
impl FromIterator < HeaderName > for AccessControlRequestHeaders { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = HeaderName > , { let flat = iter . into_iter () . map (HeaderValue :: from) . collect () ; AccessControlRequestHeaders (flat) } }
};
}
