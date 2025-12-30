// Generated macro for impl_764 (impl)
macro_rules! Depcrate_base_conversionimpl_764 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_764"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim , RStride : Dim , CStride : Dim > IntoIterator for Matrix < T , R , C , ViewStorage < 'a , T , R , C , RStride , CStride > > { type Item = & 'a T ; type IntoIter = MatrixIter < 'a , T , R , C , ViewStorage < 'a , T , R , C , RStride , CStride > > ; # [inline] fn into_iter (self) -> Self :: IntoIter { MatrixIter :: new_owned (self . data) } }
};
}
