// Generated macro for impl_273 (impl)
macro_rules! Depcrate_builder_resettableimpl_273 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_273"}
// Dependencies: {}
impl < I : Into < StyledStr > > IntoResettable < StyledStr > for I { fn into_resettable (self) -> Resettable < StyledStr > { Resettable :: Value (self . into ()) } }
};
}
