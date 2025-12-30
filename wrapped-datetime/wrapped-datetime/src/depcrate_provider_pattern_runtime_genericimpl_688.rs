// Generated macro for impl_688 (impl)
macro_rules! Depcrate_provider_pattern_runtime_genericimpl_688 {
() => {
// Module: crate::provider::pattern::runtime::generic
// Provides: {"impl_688"}
// Dependencies: {}
impl From < & reference :: GenericPattern > for GenericPattern < '_ > { fn from (input : & reference :: GenericPattern) -> Self { Self { items : ZeroVec :: alloc_from_slice (& input . items) , } } }
};
}
