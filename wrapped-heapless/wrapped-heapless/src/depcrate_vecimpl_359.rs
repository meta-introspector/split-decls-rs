// Generated macro for impl_359 (impl)
macro_rules! Depcrate_vecimpl_359 {
() => {
// Module: crate::vec
// Provides: {"impl_359"}
// Dependencies: {}
impl < 'a , T , LenT : LenType , S : VecStorage < T > + ? Sized > IntoIterator for & 'a mut VecInner < T , LenT , S > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
