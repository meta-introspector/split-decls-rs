// Generated macro for impl_323 (impl)
macro_rules! Depcrate_pattern_patternimpl_323 {
() => {
// Module: crate::pattern::pattern
// Provides: {"impl_323"}
// Dependencies: {}
impl FromStr for DateTimePattern { type Err = PatternError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_pattern_str (s) } }
};
}
