// Generated macro for impl_244 (impl)
macro_rules! Depcrateimpl_244 {
() => {
// Module: crate
// Provides: {"impl_244"}
// Dependencies: {}
impl < 'a , T : 'a , N : ArrayLength > IntoIterator for & 'a GenericArray < T , N > { type IntoIter = slice :: Iter < 'a , T > ; type Item = & 'a T ; # [inline] fn into_iter (self : & 'a GenericArray < T , N >) -> Self :: IntoIter { self . as_slice () . iter () } }
};
}
