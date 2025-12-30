// Generated macro for macro_2451 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2451 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2451"}
// Dependencies: {}
similarity_binop_impl_all ! (Mul , mul ; self : Similarity < T , R , D >, right : Point < T , D >, Output = Point < T , D >; [val val] => { let scaling = self . scaling () ; self . isometry . translation * (self . isometry . rotation . transform_point (& right) * scaling) } ; [ref val] => & self . isometry . translation * (self . isometry . rotation . transform_point (& right) * self . scaling ()) ; [val ref] => { let scaling = self . scaling () ; self . isometry . translation * (self . isometry . rotation . transform_point (right) * scaling) } ; [ref ref] => & self . isometry . translation * (self . isometry . rotation . transform_point (right) * self . scaling ()) ;) ;
};
}
