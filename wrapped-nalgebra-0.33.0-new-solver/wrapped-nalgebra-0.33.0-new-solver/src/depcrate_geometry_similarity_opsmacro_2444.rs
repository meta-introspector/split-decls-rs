// Generated macro for macro_2444 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2444 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2444"}
// Dependencies: {}
md_assign_impl_all ! (DivAssign , div_assign where T : SimdRealField for T :: Element : SimdRealField ; (U3 , U3) , (U3 , U3) const ; for ; where ; self : Similarity < T , UnitQuaternion < T >, 3 >, rhs : UnitQuaternion < T >; [val] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ;) ;
};
}
