// Generated macro for macro_2453 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2453 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2453"}
// Dependencies: {}
similarity_binop_impl_all ! (Mul , mul ; self : Similarity < T , R , D >, right : Translation < T , D >, Output = Similarity < T , R , D >; [val val] => & self * & right ; [ref val] => self * & right ; [val ref] => & self * right ; [ref ref] => { let shift = self . isometry . rotation . transform_vector (& right . vector) * self . scaling () ; Similarity :: from_parts (# [allow (clippy :: suspicious_arithmetic_impl)] Translation :: from (& self . isometry . translation . vector + shift) , self . isometry . rotation . clone () , self . scaling ()) } ;) ;
};
}
