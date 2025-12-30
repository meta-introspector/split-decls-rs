// Generated macro for impl_3105 (impl)
macro_rules! Depcrate_sparse_cs_matrix_opsimpl_3105 {
() => {
// Module: crate::sparse::cs_matrix_ops
// Provides: {"impl_3105"}
// Dependencies: {}
impl < T : Scalar + Zero + ClosedAddAssign + ClosedMulAssign , D : Dim , S : StorageMut < T , D > > Vector < T , D , S > { # [doc = " Perform a sparse axpy operation: `self = alpha * x + beta * self` operation."] pub fn axpy_cs < D2 : Dim , S2 > (& mut self , alpha : T , x : & CsVector < T , D2 , S2 > , beta : T) where S2 : CsStorage < T , D2 > , ShapeConstraint : DimEq < D , D2 > , { if beta . is_zero () { for i in 0 .. x . len () { unsafe { let k = x . data . row_index_unchecked (i) ; let y = self . vget_unchecked_mut (k) ; * y = alpha . clone () * x . data . get_value_unchecked (i) . clone () ; } } } else { * self *= beta . clone () ; for i in 0 .. x . len () { unsafe { let k = x . data . row_index_unchecked (i) ; let y = self . vget_unchecked_mut (k) ; * y += alpha . clone () * x . data . get_value_unchecked (i) . clone () ; } } } } }
};
}
