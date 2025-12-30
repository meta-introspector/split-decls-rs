// Generated macro for impl_3093 (impl)
macro_rules! Depcrate_sparse_cs_matrix_conversionimpl_3093 {
() => {
// Module: crate::sparse::cs_matrix_conversion
// Provides: {"impl_3093"}
// Dependencies: {}
impl < 'a , T : Scalar + Zero , R : Dim , C : Dim , S > From < CsMatrix < T , R , C , S > > for OMatrix < T , R , C > where S : CsStorage < T , R , C > , DefaultAllocator : Allocator < R , C > , { fn from (m : CsMatrix < T , R , C , S >) -> Self { let (nrows , ncols) = m . data . shape () ; let mut res = OMatrix :: zeros_generic (nrows , ncols) ; for j in 0 .. ncols . value () { for (i , val) in m . data . column_entries (j) { res [(i , j)] = val ; } } res } }
};
}
