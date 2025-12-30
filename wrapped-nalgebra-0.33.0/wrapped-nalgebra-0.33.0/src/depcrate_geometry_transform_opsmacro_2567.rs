// Generated macro for macro_2567 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2567 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2567"}
// Dependencies: {}
md_impl_all ! (Mul , mul where T : RealField ; (Const < D >, Const < D >) , (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) const D ; for C ; where Const < D >: DimNameAdd < U1 >, C : TCategoryMul < TAffine >, DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Rotation < T , D >, rhs : Transform < T , C , D >, Output = Transform < T , C :: Representative , D >; [val val] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . into_inner ()) ; [ref val] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . into_inner ()) ; [val ref] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . matrix ()) ; [ref ref] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . matrix ()) ;) ;
};
}
