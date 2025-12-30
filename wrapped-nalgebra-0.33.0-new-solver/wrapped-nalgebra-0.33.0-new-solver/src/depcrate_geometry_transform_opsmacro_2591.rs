// Generated macro for macro_2591 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2591 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2591"}
// Dependencies: {}
md_assign_impl_all ! (MulAssign , mul_assign where T : RealField ; (U3 , U3) , (U2 , U1) const ; for C ; where C : TCategory ; self : Transform < T , C , 2 >, rhs : UnitComplex < T >; [val] => * self . matrix_mut_unchecked () *= rhs . to_homogeneous () ; [ref] => * self . matrix_mut_unchecked () *= rhs . clone () . to_homogeneous () ;) ;
};
}
