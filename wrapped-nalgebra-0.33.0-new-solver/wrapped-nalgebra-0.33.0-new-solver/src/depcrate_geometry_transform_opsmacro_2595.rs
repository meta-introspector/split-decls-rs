// Generated macro for macro_2595 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2595 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2595"}
// Dependencies: {}
md_assign_impl_all ! (DivAssign , div_assign where T : RealField ; (U4 , U4) , (U4 , U1) const ; for C ; where C : TCategory ; self : Transform < T , C , 3 >, rhs : UnitQuaternion < T >; [val] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ;) ;
};
}
