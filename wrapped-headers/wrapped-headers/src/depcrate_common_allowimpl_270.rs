// Generated macro for impl_270 (impl)
macro_rules! Depcrate_common_allowimpl_270 {
() => {
// Module: crate::common::allow
// Provides: {"impl_270"}
// Dependencies: {}
impl FromIterator < Method > for Allow { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = Method > , { let flat = iter . into_iter () . map (| method | { method . as_str () . parse :: < HeaderValue > () . expect ("Method is a valid HeaderValue") }) . collect () ; Allow (flat) } }
};
}
