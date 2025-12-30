// Generated macro for macro_2464 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2464 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2464"}
// Dependencies: {}
similarity_from_composition_impl_all ! (Div , div ; ; self : UnitQuaternion < T >, right : Similarity < T , UnitQuaternion < T >, 3 >, Output = Similarity < T , UnitQuaternion < T >, 3 >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ;) ;
};
}
