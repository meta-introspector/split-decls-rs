// Generated macro for macro_2446 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2446 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2446"}
// Dependencies: {}
md_assign_impl_all ! (DivAssign , div_assign where T : SimdRealField for T :: Element : SimdRealField ; (U2 , U2) , (U2 , U2) const ; for ; where ; self : Similarity < T , UnitComplex < T >, 2 >, rhs : UnitComplex < T >; [val] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ;) ;
};
}
