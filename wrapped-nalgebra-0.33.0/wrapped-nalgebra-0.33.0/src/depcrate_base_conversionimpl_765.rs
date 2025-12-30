// Generated macro for impl_765 (impl)
macro_rules! Depcrate_base_conversionimpl_765 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_765"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim , S : RawStorageMut < T , R , C > > IntoIterator for & 'a mut Matrix < T , R , C , S > { type Item = & 'a mut T ; type IntoIter = MatrixIterMut < 'a , T , R , C , S > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
