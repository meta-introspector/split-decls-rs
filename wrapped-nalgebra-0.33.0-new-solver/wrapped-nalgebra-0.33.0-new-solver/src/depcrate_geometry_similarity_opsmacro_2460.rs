// Generated macro for macro_2460 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2460 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2460"}
// Dependencies: {}
similarity_from_composition_impl_all ! (Div , div ; D ; self : Rotation < T , D >, right : Similarity < T , Rotation < T , D >, D >, Output = Similarity < T , Rotation < T , D >, D >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ;) ;
};
}
