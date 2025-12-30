// Generated macro for macro_2571 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2571 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2571"}
// Dependencies: {}
md_impl_all ! (Mul , mul where T : RealField ; (U2 , U1) , (U3 , U3) const ; for C ; where C : TCategoryMul < TAffine >; self : UnitComplex < T >, rhs : Transform < T , C , 2 >, Output = Transform < T , C :: Representative , 2 >; [val val] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . into_inner ()) ; [ref val] => Self :: Output :: from_matrix_unchecked (self . clone () . to_homogeneous () * rhs . into_inner ()) ; [val ref] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . matrix ()) ; [ref ref] => Self :: Output :: from_matrix_unchecked (self . clone () . to_homogeneous () * rhs . matrix ()) ;) ;
};
}
