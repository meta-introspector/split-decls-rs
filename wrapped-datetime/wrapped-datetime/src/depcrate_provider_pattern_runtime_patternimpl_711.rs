// Generated macro for impl_711 (impl)
macro_rules! Depcrate_provider_pattern_runtime_patternimpl_711 {
() => {
// Module: crate::provider::pattern::runtime::pattern
// Provides: {"impl_711"}
// Dependencies: {}
impl From < Vec < PatternItem > > for Pattern < '_ > { fn from (items : Vec < PatternItem >) -> Self { Self { metadata : PatternMetadata :: from_items (& items) , items : ZeroVec :: alloc_from_slice (& items) , } } }
};
}
