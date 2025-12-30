// Generated macro for macro_2448 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2448 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2448"}
// Dependencies: {}
similarity_binop_impl_all ! (Div , div ; self : Similarity < T , R , D >, rhs : Isometry < T , R , D >, Output = Similarity < T , R , D >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ;) ;
};
}
