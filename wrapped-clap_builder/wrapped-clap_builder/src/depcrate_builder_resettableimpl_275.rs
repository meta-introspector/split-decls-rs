// Generated macro for impl_275 (impl)
macro_rules! Depcrate_builder_resettableimpl_275 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_275"}
// Dependencies: {}
impl < I : Into < Str > > IntoResettable < Str > for I { fn into_resettable (self) -> Resettable < Str > { Resettable :: Value (self . into ()) } }
};
}
