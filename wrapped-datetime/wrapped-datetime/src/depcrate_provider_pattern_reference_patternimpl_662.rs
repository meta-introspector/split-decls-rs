// Generated macro for impl_662 (impl)
macro_rules! Depcrate_provider_pattern_reference_patternimpl_662 {
() => {
// Module: crate::provider::pattern::reference::pattern
// Provides: {"impl_662"}
// Dependencies: {}
impl FromStr for Pattern { type Err = PatternError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Parser :: new (s) . parse () . map (Self :: from) } }
};
}
