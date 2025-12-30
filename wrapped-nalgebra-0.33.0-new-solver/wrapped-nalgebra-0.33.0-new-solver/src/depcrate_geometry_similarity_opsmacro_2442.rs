// Generated macro for macro_2442 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2442 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2442"}
// Dependencies: {}
md_assign_impl_all ! (DivAssign , div_assign where T : SimdRealField for T :: Element : SimdRealField ; (Const < D >, U1) , (Const < D >, Const < D >) const D ; for ; where ; self : Similarity < T , Rotation < T , D >, D >, rhs : Rotation < T , D >; [val] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ;) ;
};
}
