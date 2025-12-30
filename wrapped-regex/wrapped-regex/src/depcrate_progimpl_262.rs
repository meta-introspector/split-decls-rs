// Generated macro for impl_262 (impl)
macro_rules! Depcrate_progimpl_262 {
() => {
// Module: crate::prog
// Provides: {"impl_262"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Program { type Item = & 'a Inst ; type IntoIter = slice :: Iter < 'a , Inst > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
