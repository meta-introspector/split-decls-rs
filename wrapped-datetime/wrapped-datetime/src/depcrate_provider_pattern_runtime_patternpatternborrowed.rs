// Generated macro for PatternBorrowed (struct)
macro_rules! Depcrate_provider_pattern_runtime_patternPatternBorrowed {
() => {
// Module: crate::provider::pattern::runtime::pattern
// Provides: {"PatternBorrowed"}
// Dependencies: {}
# [doc = " Fully borrowed version of [`Pattern`]."] # [derive (Debug , Copy , Clone)] pub (crate) struct PatternBorrowed < 'data > { pub (crate) items : & 'data ZeroSlice < PatternItem > , pub (crate) metadata : PatternMetadata , }
};
}
