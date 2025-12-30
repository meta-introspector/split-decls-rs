// Generated macro for macro_2572 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2572 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2572"}
// Dependencies: {}
md_impl_all ! (Mul , mul where T : RealField ; (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) , (Const < D >, U1) const D ; for C , R ; where Const < D >: DimNameAdd < U1 >, C : TCategoryMul < TAffine >, R : SubsetOf < OMatrix < T , DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >> >, DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Transform < T , C , D >, rhs : Isometry < T , R , D >, Output = Transform < T , C :: Representative , D >; [val val] => Self :: Output :: from_matrix_unchecked (self . into_inner () * rhs . to_homogeneous ()) ; [ref val] => Self :: Output :: from_matrix_unchecked (self . matrix () * rhs . to_homogeneous ()) ; [val ref] => Self :: Output :: from_matrix_unchecked (self . into_inner () * rhs . to_homogeneous ()) ; [ref ref] => Self :: Output :: from_matrix_unchecked (self . matrix () * rhs . to_homogeneous ()) ;) ;
};
}
