// Generated macro for impl_321 (impl)
macro_rules! Depcrate_common_connectionimpl_321 {
() => {
// Module: crate::common::connection
// Provides: {"impl_321"}
// Dependencies: {}
impl FromIterator < HeaderName > for Connection { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = HeaderName > , { let flat = iter . into_iter () . map (HeaderValue :: from) . collect () ; Connection (flat) } }
};
}
