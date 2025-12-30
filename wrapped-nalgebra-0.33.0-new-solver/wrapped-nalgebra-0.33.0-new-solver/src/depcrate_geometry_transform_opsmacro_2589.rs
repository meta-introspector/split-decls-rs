// Generated macro for macro_2589 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2589 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2589"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign where T : RealField ; (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) , (Const < D >, Const < D >) const D ; for C ; where Const < D >: DimNameAdd < U1 >, C : TCategory , DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Transform < T , C , D >, rhs : Rotation < T , D >; [val] => * self . matrix_mut_unchecked () *= rhs . to_homogeneous () ; [ref] => * self . matrix_mut_unchecked () *= rhs . to_homogeneous () ;) ;
};
}
