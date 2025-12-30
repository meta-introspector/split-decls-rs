// Generated macro for impl_710 (impl)
macro_rules! Depcrate_provider_pattern_runtime_patternimpl_710 {
() => {
// Module: crate::provider::pattern::runtime::pattern
// Provides: {"impl_710"}
// Dependencies: {}
impl < 'data > PatternBorrowed < 'data > { pub (crate) const DEFAULT : PatternBorrowed < 'static > = PatternBorrowed { items : ZeroSlice :: new_empty () , metadata : PatternMetadata :: DEFAULT , } ; pub (crate) fn as_pattern (self) -> Pattern < 'data > { Pattern { items : self . items . as_zerovec () , metadata : self . metadata , } } }
};
}
