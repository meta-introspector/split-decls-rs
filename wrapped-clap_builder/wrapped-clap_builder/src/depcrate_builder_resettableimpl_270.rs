// Generated macro for impl_270 (impl)
macro_rules! Depcrate_builder_resettableimpl_270 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_270"}
// Dependencies: {}
impl < I : Into < ValueRange > > IntoResettable < ValueRange > for I { fn into_resettable (self) -> Resettable < ValueRange > { Resettable :: Value (self . into ()) } }
};
}
