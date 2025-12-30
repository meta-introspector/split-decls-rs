// Generated macro for macro_2573 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2573 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2573"}
// Dependencies: {}
md_impl_all ! (Mul , mul where T : RealField ; (Const < D >, U1) , (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) const D ; for C , R ; where Const < D >: DimNameAdd < U1 >, C : TCategoryMul < TAffine >, R : SubsetOf < OMatrix < T , DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >> >, DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Isometry < T , R , D >, rhs : Transform < T , C , D >, Output = Transform < T , C :: Representative , D >; [val val] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . into_inner ()) ; [ref val] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . into_inner ()) ; [val ref] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . matrix ()) ; [ref ref] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . matrix ()) ;) ;
};
}
