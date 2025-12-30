// Generated macro for macro_2434 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2434 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2434"}
// Dependencies: {}
similarity_binop_impl_all ! (Mul , mul ; self : Similarity < T , R , D >, rhs : Similarity < T , R , D >, Output = Similarity < T , R , D >; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => { let mut res = self * & rhs . isometry ; res . prepend_scaling_mut (rhs . scaling ()) ; res } ;) ;
};
}
