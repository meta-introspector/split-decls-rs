// Generated macro for macro_2596 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2596 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2596"}
// Dependencies: {}
md_assign_impl_all ! (DivAssign , div_assign where T : RealField ; (U3 , U3) , (U2 , U1) const ; for C ; where C : TCategory ; self : Transform < T , C , 2 >, rhs : UnitComplex < T >; [val] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ;) ;
};
}
