// Generated macro for impl_358 (impl)
macro_rules! Depcrate_vecimpl_358 {
() => {
// Module: crate::vec
// Provides: {"impl_358"}
// Dependencies: {}
impl < 'a , T , LenT : LenType , S : VecStorage < T > + ? Sized > IntoIterator for & 'a VecInner < T , LenT , S > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
