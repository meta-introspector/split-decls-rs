// Generated macro for impl_369 (impl)
macro_rules! Depcrate_vecimpl_369 {
() => {
// Module: crate::vec
// Provides: {"impl_369"}
// Dependencies: {}
impl < T , LenT : LenType , const N : usize > IntoIterator for Vec < T , N , LenT > { type Item = T ; type IntoIter = IntoIter < T , N , LenT > ; fn into_iter (self) -> Self :: IntoIter { IntoIter { vec : self , next : LenT :: ZERO , } } }
};
}
