// Generated macro for impl_712 (impl)
macro_rules! Depcrate_provider_pattern_runtime_patternimpl_712 {
() => {
// Module: crate::provider::pattern::runtime::pattern
// Provides: {"impl_712"}
// Dependencies: {}
impl FromIterator < PatternItem > for Pattern < '_ > { fn from_iter < T : IntoIterator < Item = PatternItem > > (iter : T) -> Self { let items = iter . into_iter () . collect :: < ZeroVec < PatternItem > > () ; Self { metadata : PatternMetadata :: from_iter_items (items . iter ()) , items , } } }
};
}
