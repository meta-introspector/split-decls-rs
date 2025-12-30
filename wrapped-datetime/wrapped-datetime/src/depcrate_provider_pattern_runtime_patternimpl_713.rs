// Generated macro for impl_713 (impl)
macro_rules! Depcrate_provider_pattern_runtime_patternimpl_713 {
() => {
// Module: crate::provider::pattern::runtime::pattern
// Provides: {"impl_713"}
// Dependencies: {}
impl From < & reference :: Pattern > for Pattern < '_ > { fn from (input : & reference :: Pattern) -> Self { Self { items : ZeroVec :: alloc_from_slice (& input . items) , metadata : PatternMetadata :: from_time_granularity (input . time_granularity) , } } }
};
}
