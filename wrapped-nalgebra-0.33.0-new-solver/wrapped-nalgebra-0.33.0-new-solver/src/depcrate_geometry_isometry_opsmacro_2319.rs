// Generated macro for macro_2319 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2319 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2319"}
// Dependencies: {}
isometry_binop_impl_all ! (Div , div ; self : Isometry < T , R , D >, rhs : Isometry < T , R , D >, Output = Isometry < T , R , D >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ;) ;
};
}
