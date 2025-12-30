// Generated macro for impl_766 (impl)
macro_rules! Depcrate_base_conversionimpl_766 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_766"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim , RStride : Dim , CStride : Dim > IntoIterator for Matrix < T , R , C , ViewStorageMut < 'a , T , R , C , RStride , CStride > > { type Item = & 'a mut T ; type IntoIter = MatrixIterMut < 'a , T , R , C , ViewStorageMut < 'a , T , R , C , RStride , CStride > > ; # [inline] fn into_iter (self) -> Self :: IntoIter { MatrixIterMut :: new_owned_mut (self . data) } }
};
}
