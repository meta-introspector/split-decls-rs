// Generated macro for macro_2587 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2587 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2587"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign where T : RealField ; (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) , (Const < D >, U1) const D ; for C , R ; where Const < D >: DimNameAdd < U1 >, C : TCategory , R : SubsetOf < OMatrix < T , DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >> >, DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Transform < T , C , D >, rhs : Isometry < T , R , D >; [val] => * self . matrix_mut_unchecked () *= rhs . to_homogeneous () ; [ref] => * self . matrix_mut_unchecked () *= rhs . to_homogeneous () ;) ;
};
}
