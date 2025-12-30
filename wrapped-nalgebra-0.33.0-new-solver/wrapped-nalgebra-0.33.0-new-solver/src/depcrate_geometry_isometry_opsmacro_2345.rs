// Generated macro for macro_2345 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2345 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2345"}
// Dependencies: {}
isometry_from_composition_impl_all ! (Div , div ; ; self : UnitQuaternion < T >, right : Isometry < T , UnitQuaternion < T >, 3 >, Output = Isometry < T , UnitQuaternion < T >, 3 >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ;) ;
};
}
