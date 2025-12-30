// Generated macro for impl_643 (impl)
macro_rules! Depcrate_common_varyimpl_643 {
() => {
// Module: crate::common::vary
// Provides: {"impl_643"}
// Dependencies: {}
impl From < HeaderName > for Vary { fn from (name : HeaderName) -> Self { Vary (HeaderValue :: from (name) . into ()) } }
};
}
