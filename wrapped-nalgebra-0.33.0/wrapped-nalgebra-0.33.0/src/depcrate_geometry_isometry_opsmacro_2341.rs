// Generated macro for macro_2341 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2341 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2341"}
// Dependencies: {}
isometry_from_composition_impl_all ! (Div , div ; D ; self : Rotation < T , D >, right : Isometry < T , Rotation < T , D >, D >, Output = Isometry < T , Rotation < T , D >, D >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ;) ;
};
}
