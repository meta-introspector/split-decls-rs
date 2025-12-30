// Generated macro for macro_2565 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2565 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2565"}
// Dependencies: {}
md_impl_all ! (Mul , mul where T : RealField ; (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) , (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) const D ; for CA , CB ; where Const < D >: DimNameAdd < U1 >, CA : TCategoryMul < CB >, CB : TCategory , DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Transform < T , CA , D >, rhs : Transform < T , CB , D >, Output = Transform < T , CA :: Representative , D >; [val val] => Self :: Output :: from_matrix_unchecked (self . into_inner () * rhs . into_inner ()) ; [ref val] => Self :: Output :: from_matrix_unchecked (self . matrix () * rhs . into_inner ()) ; [val ref] => Self :: Output :: from_matrix_unchecked (self . into_inner () * rhs . matrix ()) ; [ref ref] => Self :: Output :: from_matrix_unchecked (self . matrix () * rhs . matrix ()) ;) ;
};
}
