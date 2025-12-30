// Generated macro for impl_924 (impl)
macro_rules! Depcrate_base_matriximpl_924 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_924"}
// Dependencies: {}
impl < T : Scalar + Zero + One , D : DimAdd < U1 > + IsNotStaticOne , S : RawStorage < T , D , D > > Matrix < T , D , D , S > { # [doc = " Yields the homogeneous matrix for this matrix, i.e., appending an additional dimension and"] # [doc = " and setting the diagonal element to `1`."] # [inline] # [must_use] pub fn to_homogeneous (& self) -> OMatrix < T , DimSum < D , U1 > , DimSum < D , U1 > > where DefaultAllocator : Allocator < DimSum < D , U1 > , DimSum < D , U1 > > , { assert ! (self . is_square () , "Only square matrices can currently be transformed to homogeneous coordinates.") ; let dim = DimSum :: < D , U1 > :: from_usize (self . nrows () + 1) ; let mut res = OMatrix :: identity_generic (dim , dim) ; res . generic_view_mut :: < D , D > ((0 , 0) , self . shape_generic ()) . copy_from (self) ; res } }
};
}
