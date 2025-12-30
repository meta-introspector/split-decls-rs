// Generated macro for impl_715 (impl)
macro_rules! Depcrate_provider_pattern_runtime_patternimpl_715 {
() => {
// Module: crate::provider::pattern::runtime::pattern
// Provides: {"impl_715"}
// Dependencies: {}
impl FromStr for Pattern < '_ > { type Err = PatternError ; fn from_str (input : & str) -> Result < Self , Self :: Err > { let reference = reference :: Pattern :: from_str (input) ? ; Ok (Self :: from (& reference)) } }
};
}
