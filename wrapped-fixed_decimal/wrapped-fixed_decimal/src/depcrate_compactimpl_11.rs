// Generated macro for impl_11 (impl)
macro_rules! Depcrate_compactimpl_11 {
() => {
// Module: crate::compact
// Provides: {"impl_11"}
// Dependencies: {}
impl FromStr for CompactDecimal { type Err = ParseError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
