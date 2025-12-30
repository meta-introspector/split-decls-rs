// Generated macro for macro_2452 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2452 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2452"}
// Dependencies: {}
similarity_binop_impl_all ! (Mul , mul ; self : Similarity < T , R , D >, right : SVector < T , D >, Output = SVector < T , D >; [val val] => self . isometry . rotation . transform_vector (& right) * self . scaling () ; [ref val] => self . isometry . rotation . transform_vector (& right) * self . scaling () ; [val ref] => self . isometry . rotation . transform_vector (right) * self . scaling () ; [ref ref] => self . isometry . rotation . transform_vector (right) * self . scaling () ;) ;
};
}
