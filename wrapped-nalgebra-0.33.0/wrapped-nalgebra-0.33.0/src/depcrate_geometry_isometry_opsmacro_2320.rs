// Generated macro for macro_2320 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2320 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2320"}
// Dependencies: {}
isometry_binop_assign_impl_all ! (MulAssign , mul_assign ; self : Isometry < T , R , D >, rhs : Translation < T , D >; [val] => * self *= & rhs ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { let shift = self . rotation . transform_vector (& rhs . vector) ; self . translation . vector += shift ; } ;) ;
};
}
