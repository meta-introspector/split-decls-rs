// Generated macro for macro_2454 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2454 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2454"}
// Dependencies: {}
similarity_binop_impl_all ! (Mul , mul ; self : Translation < T , D >, right : Similarity < T , R , D >, Output = Similarity < T , R , D >; [val val] => { let scaling = right . scaling () ; Similarity :: from_isometry (self * right . isometry , scaling) } ; [ref val] => { let scaling = right . scaling () ; Similarity :: from_isometry (self * right . isometry , scaling) } ; [val ref] => Similarity :: from_isometry (self * & right . isometry , right . scaling ()) ; [ref ref] => Similarity :: from_isometry (self * & right . isometry , right . scaling ()) ;) ;
};
}
