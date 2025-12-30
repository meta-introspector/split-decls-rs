// Generated macro for macro_2581 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2581 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2581"}
// Dependencies: {}
md_impl_all ! (Div , div where T : RealField ; (U4 , U4) , (U4 , U1) const ; for C ; where C : TCategoryMul < TAffine >; self : Transform < T , C , 3 >, rhs : UnitQuaternion < T >, Output = Transform < T , C :: Representative , 3 >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ;) ;
};
}
