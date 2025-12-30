// Generated macro for macro_1463 (macro)
macro_rules! Depcrate_geometry_rotation_opsmacro_1463 {
() => {
// Module: crate::geometry::rotation_ops
// Provides: {"macro_1463"}
// Dependencies: {}
md_impl_all ! (Div , div ; (Const < D >, Const < D >) , (Const < D >, Const < D >) const D ; for ; where ; self : Rotation < T , D >, right : Rotation < T , D >, Output = Rotation < T , D >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ;) ;
};
}
