// Generated macro for macro_2440 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2440 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2440"}
// Dependencies: {}
similarity_binop_assign_impl_all ! (DivAssign , div_assign ; self : Similarity < T , R , D >, rhs : Isometry < T , R , D >; [val] => * self /= & rhs ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ;) ;
};
}
