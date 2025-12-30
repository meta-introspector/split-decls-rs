// Generated macro for macro_2326 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2326 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2326"}
// Dependencies: {}
md_assign_impl_all ! (DivAssign , div_assign where T : SimdRealField for T :: Element : SimdRealField ; (U3 , U3) , (U3 , U3) const ; for ; where ; self : Isometry < T , UnitQuaternion < T >, 3 >, rhs : UnitQuaternion < T >; [val] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ;) ;
};
}
