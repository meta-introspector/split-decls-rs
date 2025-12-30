// Generated macro for impl_245 (impl)
macro_rules! Depcrate_common_access_control_request_headersimpl_245 {
() => {
// Module: crate::common::access_control_request_headers
// Provides: {"impl_245"}
// Dependencies: {}
impl AccessControlRequestHeaders { # [doc = " Returns an iterator over `HeaderName`s contained within."] pub fn iter (& self) -> impl Iterator < Item = HeaderName > + '_ { self . 0 . iter () . filter_map (| s | s . parse () . ok ()) } }
};
}
