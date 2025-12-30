// Generated macro for impl_170 (impl)
macro_rules! Depcrateimpl_170 {
() => {
// Module: crate
// Provides: {"impl_170"}
// Dependencies: {}
impl < 'a , T , const N : usize > IntoIterator for & 'a SmallVec < T , N > { type IntoIter = core :: slice :: Iter < 'a , T > ; type Item = & 'a T ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
