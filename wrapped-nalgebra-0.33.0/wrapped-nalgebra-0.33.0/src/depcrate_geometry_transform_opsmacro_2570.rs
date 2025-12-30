// Generated macro for macro_2570 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2570 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2570"}
// Dependencies: {}
md_impl_all ! (Mul , mul where T : RealField ; (U4 , U1) , (U4 , U4) const ; for C ; where C : TCategoryMul < TAffine >; self : UnitQuaternion < T >, rhs : Transform < T , C , 3 >, Output = Transform < T , C :: Representative , 3 >; [val val] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . into_inner ()) ; [ref val] => Self :: Output :: from_matrix_unchecked (self . clone () . to_homogeneous () * rhs . into_inner ()) ; [val ref] => Self :: Output :: from_matrix_unchecked (self . to_homogeneous () * rhs . matrix ()) ; [ref ref] => Self :: Output :: from_matrix_unchecked (self . clone () . to_homogeneous () * rhs . matrix ()) ;) ;
};
}
