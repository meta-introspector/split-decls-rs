// Generated macro for impl_661 (impl)
macro_rules! Depcrate_provider_pattern_reference_patternimpl_661 {
() => {
// Module: crate::provider::pattern::reference::pattern
// Provides: {"impl_661"}
// Dependencies: {}
impl From < & str > for Pattern { fn from (items : & str) -> Self { Self { time_granularity : TimeGranularity :: default () , items : items . chars () . map (| ch | ch . into ()) . collect () , } } }
};
}
