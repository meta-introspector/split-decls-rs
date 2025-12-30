// Generated macro for impl_171 (impl)
macro_rules! Depcrateimpl_171 {
() => {
// Module: crate
// Provides: {"impl_171"}
// Dependencies: {}
impl < 'a , T , const N : usize > IntoIterator for & 'a mut SmallVec < T , N > { type IntoIter = core :: slice :: IterMut < 'a , T > ; type Item = & 'a mut T ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
