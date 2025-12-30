// Generated macro for impl_73 (impl)
macro_rules! Depcrate_iterimpl_73 {
() => {
// Module: crate::iter
// Provides: {"impl_73"}
// Dependencies: {}
impl < T : Clone , N : ArrayLength > Clone for GenericArrayIter < T , N > { fn clone (& self) -> Self { let mut array = unsafe { ptr :: read (& self . array) } ; let mut index_back = 0 ; for (dst , src) in array . as_mut_slice () . iter_mut () . zip (self . as_slice ()) { unsafe { ptr :: write (dst , src . clone ()) } ; index_back += 1 ; } GenericArrayIter { array , index : 0 , index_back , } } }
};
}
