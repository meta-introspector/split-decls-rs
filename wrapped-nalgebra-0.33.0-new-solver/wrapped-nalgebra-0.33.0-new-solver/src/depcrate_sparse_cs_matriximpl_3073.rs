// Generated macro for impl_3073 (impl)
macro_rules! Depcrate_sparse_cs_matriximpl_3073 {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"impl_3073"}
// Dependencies: {}
impl < T : Scalar , R : Dim , C : Dim , S : CsStorageMut < T , R , C > > CsMatrix < T , R , C , S > { # [doc = " Iterator through all the mutable values of this sparse matrix."] # [inline] pub fn values_mut (& mut self) -> impl Iterator < Item = & mut T > { self . data . values_mut () } }
};
}
