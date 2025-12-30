// Generated macro for impl_660 (impl)
macro_rules! Depcrate_provider_pattern_reference_patternimpl_660 {
() => {
// Module: crate::provider::pattern::reference::pattern
// Provides: {"impl_660"}
// Dependencies: {}
impl From < Vec < PatternItem > > for Pattern { fn from (items : Vec < PatternItem >) -> Self { Self { time_granularity : items . iter () . copied () . map (Into :: into) . max () . unwrap_or_default () , items , } } }
};
}
