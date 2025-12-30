// Generated macro for macro_2322 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2322 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2322"}
// Dependencies: {}
isometry_binop_assign_impl_all ! (DivAssign , div_assign ; self : Isometry < T , R , D >, rhs : Isometry < T , R , D >; [val] => * self /= & rhs ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ;) ;
};
}
