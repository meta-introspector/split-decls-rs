// Generated macro for impl_271 (impl)
macro_rules! Depcrate_builder_resettableimpl_271 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_271"}
// Dependencies: {}
impl < I : Into < ValueParser > > IntoResettable < ValueParser > for I { fn into_resettable (self) -> Resettable < ValueParser > { Resettable :: Value (self . into ()) } }
};
}
