// Generated macro for impl_7 (impl)
macro_rules! Depcrate_capturesimpl_7 {
() => {
// Module: crate::captures
// Provides: {"impl_7"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Captures { type Item = & 'a [u8] ; type IntoIter = MatchIter < 'a > ; fn into_iter (self) -> Self :: IntoIter { MatchIter :: new (self) } }
};
}
