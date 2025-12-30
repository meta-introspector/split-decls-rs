// Generated macro for impl_3108 (impl)
macro_rules! Depcrate_sparse_cs_matrix_opsimpl_3108 {
() => {
// Module: crate::sparse::cs_matrix_ops
// Provides: {"impl_3108"}
// Dependencies: {}
impl < 'a , 'b , T , R , C , S > Mul < T > for CsMatrix < T , R , C , S > where T : Scalar + ClosedAddAssign + ClosedMulAssign + Zero , R : Dim , C : Dim , S : CsStorageMut < T , R , C > , { type Output = Self ; fn mul (mut self , rhs : T) -> Self :: Output { for e in self . values_mut () { * e *= rhs . clone () } self } }
};
}
