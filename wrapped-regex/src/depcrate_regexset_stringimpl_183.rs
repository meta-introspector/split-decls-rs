// Generated macro for impl_183 (impl)
macro_rules! Depcrate_regexset_stringimpl_183 {
() => {
// Module: crate::regexset::string
// Provides: {"impl_183"}
// Dependencies: {}
impl IntoIterator for SetMatches { type IntoIter = SetMatchesIntoIter ; type Item = usize ; fn into_iter (self) -> Self :: IntoIter { let it = 0 .. self . 0 . capacity () ; SetMatchesIntoIter { patset : self . 0 , it } } }
};
}
