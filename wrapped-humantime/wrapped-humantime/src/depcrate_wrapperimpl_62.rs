// Generated macro for impl_62 (impl)
macro_rules! Depcrate_wrapperimpl_62 {
() => {
// Module: crate::wrapper
// Provides: {"impl_62"}
// Dependencies: {}
impl FromStr for Duration { type Err = duration :: Error ; fn from_str (s : & str) -> Result < Duration , Self :: Err > { parse_duration (s) . map (Duration) } }
};
}
