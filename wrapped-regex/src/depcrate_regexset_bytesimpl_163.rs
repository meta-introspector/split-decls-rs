// Generated macro for impl_163 (impl)
macro_rules! Depcrate_regexset_bytesimpl_163 {
() => {
// Module: crate::regexset::bytes
// Provides: {"impl_163"}
// Dependencies: {}
impl IntoIterator for SetMatches { type IntoIter = SetMatchesIntoIter ; type Item = usize ; fn into_iter (self) -> Self :: IntoIter { let it = 0 .. self . 0 . capacity () ; SetMatchesIntoIter { patset : self . 0 , it } } }
};
}
