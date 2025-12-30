// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl < T , const CAP : usize , B : Behavior > IntoIterator for ArrayDeque < T , CAP , B > { type Item = T ; type IntoIter = IntoIter < T , CAP , B > ; fn into_iter (self) -> Self :: IntoIter { IntoIter { inner : self } } }
};
}
