// Generated macro for macro_2447 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2447 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2447"}
// Dependencies: {}
similarity_binop_impl_all ! (Mul , mul ; self : Similarity < T , R , D >, rhs : Isometry < T , R , D >, Output = Similarity < T , R , D >; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => { let shift = self . isometry . rotation . transform_vector (& rhs . translation . vector) * self . scaling () ; Similarity :: from_parts (# [allow (clippy :: suspicious_arithmetic_impl)] Translation :: from (& self . isometry . translation . vector + shift) , self . isometry . rotation . clone () * rhs . rotation . clone () , self . scaling ()) } ;) ;
};
}
