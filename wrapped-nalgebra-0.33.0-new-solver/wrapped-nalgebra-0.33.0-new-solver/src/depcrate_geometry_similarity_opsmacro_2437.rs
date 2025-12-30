// Generated macro for macro_2437 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2437 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2437"}
// Dependencies: {}
similarity_binop_assign_impl_all ! (MulAssign , mul_assign ; self : Similarity < T , R , D >, rhs : Similarity < T , R , D >; [val] => * self *= & rhs ; [ref] => { * self *= & rhs . isometry ; self . prepend_scaling_mut (rhs . scaling ()) ; } ;) ;
};
}
