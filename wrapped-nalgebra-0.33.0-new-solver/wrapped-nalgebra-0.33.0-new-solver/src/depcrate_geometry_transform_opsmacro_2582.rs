// Generated macro for macro_2582 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2582 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2582"}
// Dependencies: {}
md_impl_all ! (Div , div where T : RealField ; (U4 , U1) , (U4 , U4) const ; for C ; where C : TCategoryMul < TAffine >; self : UnitQuaternion < T >, rhs : Transform < T , C , 3 >, Output = Transform < T , C :: Representative , 3 >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self . inverse () * rhs } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self . inverse () * rhs } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self . inverse () * rhs } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self . inverse () * rhs } ;) ;
};
}
