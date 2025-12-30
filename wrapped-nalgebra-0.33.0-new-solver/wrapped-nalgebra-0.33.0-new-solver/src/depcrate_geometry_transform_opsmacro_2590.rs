// Generated macro for macro_2590 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2590 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2590"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign where T : RealField ; (U4 , U4) , (U4 , U1) const ; for C ; where C : TCategory ; self : Transform < T , C , 3 >, rhs : UnitQuaternion < T >; [val] => * self . matrix_mut_unchecked () *= rhs . to_homogeneous () ; [ref] => * self . matrix_mut_unchecked () *= rhs . clone () . to_homogeneous () ;) ;
};
}
