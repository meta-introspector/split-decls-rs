// Generated macro for impl_245 (impl)
macro_rules! Depcrateimpl_245 {
() => {
// Module: crate
// Provides: {"impl_245"}
// Dependencies: {}
impl < 'a , T : 'a , N : ArrayLength > IntoIterator for & 'a mut GenericArray < T , N > { type IntoIter = slice :: IterMut < 'a , T > ; type Item = & 'a mut T ; # [inline] fn into_iter (self : & 'a mut GenericArray < T , N >) -> Self :: IntoIter { self . as_mut_slice () . iter_mut () } }
};
}
