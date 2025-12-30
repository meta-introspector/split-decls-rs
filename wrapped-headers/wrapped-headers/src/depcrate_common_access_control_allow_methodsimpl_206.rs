// Generated macro for impl_206 (impl)
macro_rules! Depcrate_common_access_control_allow_methodsimpl_206 {
() => {
// Module: crate::common::access_control_allow_methods
// Provides: {"impl_206"}
// Dependencies: {}
impl FromIterator < Method > for AccessControlAllowMethods { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = Method > , { let methods = iter . into_iter () . map (| method | { method . as_str () . parse :: < HeaderValue > () . expect ("Method is a valid HeaderValue") }) . collect () ; AccessControlAllowMethods (methods) } }
};
}
