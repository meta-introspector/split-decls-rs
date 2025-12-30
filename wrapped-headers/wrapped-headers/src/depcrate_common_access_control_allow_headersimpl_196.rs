// Generated macro for impl_196 (impl)
macro_rules! Depcrate_common_access_control_allow_headersimpl_196 {
() => {
// Module: crate::common::access_control_allow_headers
// Provides: {"impl_196"}
// Dependencies: {}
impl AccessControlAllowHeaders { # [doc = " Returns an iterator over `HeaderName`s contained within."] pub fn iter (& self) -> impl Iterator < Item = HeaderName > + '_ { self . 0 . iter () . map (| s | s . parse () . ok ()) . take_while (| val | val . is_some ()) . flatten () } }
};
}
