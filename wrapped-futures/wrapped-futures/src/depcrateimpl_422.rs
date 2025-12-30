// Generated macro for impl_422 (impl)
macro_rules! Depcrateimpl_422 {
() => {
// Module: crate
// Provides: {"impl_422"}
// Dependencies: {}
impl < F : Future > IntoFuture for F { type Future = F ; type Item = F :: Item ; type Error = F :: Error ; fn into_future (self) -> F { self } }
};
}
