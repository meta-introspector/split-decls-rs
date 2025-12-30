// Generated macro for macro_2439 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2439 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2439"}
// Dependencies: {}
similarity_binop_assign_impl_all ! (MulAssign , mul_assign ; self : Similarity < T , R , D >, rhs : Isometry < T , R , D >; [val] => * self *= & rhs ; [ref] => { let shift = self . isometry . rotation . transform_vector (& rhs . translation . vector) * self . scaling () ; self . isometry . translation . vector += shift ; self . isometry . rotation *= rhs . rotation . clone () ; } ;) ;
};
}
