// Generated macro for impl_45 (impl)
macro_rules! Depcrate_rangeimpl_45 {
() => {
// Module: crate::range
// Provides: {"impl_45"}
// Dependencies: {}
impl FromStr for MediaRange { type Err = InvalidMime ; fn from_str (s : & str) -> Result < MediaRange , Self :: Err > { MediaRange :: parse (s) } }
};
}
