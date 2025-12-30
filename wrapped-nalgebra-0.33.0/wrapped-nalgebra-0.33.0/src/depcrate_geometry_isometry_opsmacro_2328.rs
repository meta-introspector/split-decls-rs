// Generated macro for macro_2328 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2328 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2328"}
// Dependencies: {}
md_assign_impl_all ! (DivAssign , div_assign where T : SimdRealField for T :: Element : SimdRealField ; (U2 , U2) , (U2 , U2) const ; for ; where ; self : Isometry < T , UnitComplex < T >, 2 >, rhs : UnitComplex < T >; [val] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ;) ;
};
}
