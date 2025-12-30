// Generated macro for impl_272 (impl)
macro_rules! Depcrate_builder_resettableimpl_272 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_272"}
// Dependencies: {}
impl < I : Into < String > > IntoResettable < String > for I { fn into_resettable (self) -> Resettable < String > { Resettable :: Value (self . into ()) } }
};
}
