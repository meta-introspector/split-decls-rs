// Generated macro for impl_229 (impl)
macro_rules! Depcrate_common_access_control_expose_headersimpl_229 {
() => {
// Module: crate::common::access_control_expose_headers
// Provides: {"impl_229"}
// Dependencies: {}
impl AccessControlExposeHeaders { # [doc = " Returns an iterator over `HeaderName`s contained within."] pub fn iter (& self) -> impl Iterator < Item = HeaderName > + '_ { self . 0 . iter () . filter_map (| s | s . parse () . ok ()) } }
};
}
