// Generated macro for macro_2569 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2569 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2569"}
// Dependencies: {}
md_impl_all ! (Mul , mul where T : RealField ; (U3 , U3) , (U2 , U1) const ; for C ; where C : TCategoryMul < TAffine >; self : Transform < T , C , 2 >, rhs : UnitComplex < T >, Output = Transform < T , C :: Representative , 2 >; [val val] => Self :: Output :: from_matrix_unchecked (self . into_inner () * rhs . to_homogeneous ()) ; [ref val] => Self :: Output :: from_matrix_unchecked (self . matrix () * rhs . to_homogeneous ()) ; [val ref] => Self :: Output :: from_matrix_unchecked (self . into_inner () * rhs . clone () . to_homogeneous ()) ; [ref ref] => Self :: Output :: from_matrix_unchecked (self . matrix () * rhs . clone () . to_homogeneous ()) ;) ;
};
}
