// Generated macro for macro_2585 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2585 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2585"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign where T : RealField ; (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) , (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) const D ; for CA , CB ; where Const < D >: DimNameAdd < U1 >, CA : TCategory , CB : SubTCategoryOf < CA >, DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Transform < T , CA , D >, rhs : Transform < T , CB , D >; [val] => * self . matrix_mut_unchecked () *= rhs . into_inner () ; [ref] => * self . matrix_mut_unchecked () *= rhs . matrix () ;) ;
};
}
