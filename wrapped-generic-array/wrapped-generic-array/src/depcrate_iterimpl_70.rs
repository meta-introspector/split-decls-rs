// Generated macro for impl_70 (impl)
macro_rules! Depcrate_iterimpl_70 {
() => {
// Module: crate::iter
// Provides: {"impl_70"}
// Dependencies: {}
impl < T , N : ArrayLength > IntoIterator for GenericArray < T , N > { type Item = T ; type IntoIter = GenericArrayIter < T , N > ; # [inline] fn into_iter (self) -> Self :: IntoIter { GenericArrayIter { array : ManuallyDrop :: new (self) , index : 0 , index_back : N :: USIZE , } } }
};
}
