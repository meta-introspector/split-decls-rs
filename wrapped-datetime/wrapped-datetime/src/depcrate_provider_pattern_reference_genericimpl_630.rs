// Generated macro for impl_630 (impl)
macro_rules! Depcrate_provider_pattern_reference_genericimpl_630 {
() => {
// Module: crate::provider::pattern::reference::generic
// Provides: {"impl_630"}
// Dependencies: {}
impl FromStr for GenericPattern { type Err = PatternError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Parser :: new (s) . parse_generic () . map (Self :: from) } }
};
}
