// Generated macro for impl_276 (impl)
macro_rules! Depcrate_builder_resettableimpl_276 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_276"}
// Dependencies: {}
impl < I : Into < crate :: Id > > IntoResettable < crate :: Id > for I { fn into_resettable (self) -> Resettable < crate :: Id > { Resettable :: Value (self . into ()) } }
};
}
