// Generated macro for macro_2436 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2436 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2436"}
// Dependencies: {}
similarity_binop_assign_impl_all ! (MulAssign , mul_assign ; self : Similarity < T , R , D >, rhs : Translation < T , D >; [val] => * self *= & rhs ; [ref] => { let shift = self . isometry . rotation . transform_vector (& rhs . vector) * self . scaling () ; self . isometry . translation . vector += shift ; } ;) ;
};
}
