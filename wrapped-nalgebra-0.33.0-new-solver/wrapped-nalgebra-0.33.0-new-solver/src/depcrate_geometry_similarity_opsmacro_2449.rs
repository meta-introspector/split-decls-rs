// Generated macro for macro_2449 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2449 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2449"}
// Dependencies: {}
similarity_binop_impl_all ! (Mul , mul ; self : Isometry < T , R , D >, rhs : Similarity < T , R , D >, Output = Similarity < T , R , D >; [val val] => { let scaling = rhs . scaling () ; Similarity :: from_isometry (self * rhs . isometry , scaling) } ; [ref val] => { let scaling = rhs . scaling () ; Similarity :: from_isometry (self * rhs . isometry , scaling) } ; [val ref] => { let scaling = rhs . scaling () ; Similarity :: from_isometry (self * & rhs . isometry , scaling) } ; [ref ref] => { let scaling = rhs . scaling () ; Similarity :: from_isometry (self * & rhs . isometry , scaling) } ;) ;
};
}
