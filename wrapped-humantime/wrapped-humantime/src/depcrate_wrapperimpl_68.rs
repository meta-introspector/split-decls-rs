// Generated macro for impl_68 (impl)
macro_rules! Depcrate_wrapperimpl_68 {
() => {
// Module: crate::wrapper
// Provides: {"impl_68"}
// Dependencies: {}
impl FromStr for Timestamp { type Err = date :: Error ; fn from_str (s : & str) -> Result < Timestamp , Self :: Err > { parse_rfc3339_weak (s) . map (Timestamp) } }
};
}
