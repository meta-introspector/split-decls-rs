// Generated macro for impl_690 (impl)
macro_rules! Depcrate_provider_pattern_runtime_genericimpl_690 {
() => {
// Module: crate::provider::pattern::runtime::generic
// Provides: {"impl_690"}
// Dependencies: {}
impl FromStr for GenericPattern < '_ > { type Err = PatternError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let reference = reference :: GenericPattern :: from_str (s) ? ; Ok (Self :: from (& reference)) } }
};
}
