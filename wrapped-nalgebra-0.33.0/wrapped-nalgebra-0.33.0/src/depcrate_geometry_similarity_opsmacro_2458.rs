// Generated macro for macro_2458 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2458 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2458"}
// Dependencies: {}
similarity_from_composition_impl_all ! (Mul , mul ; D ; self : Rotation < T , D >, right : Similarity < T , Rotation < T , D >, D >, Output = Similarity < T , Rotation < T , D >, D >; [val val] => & self * & right ; [ref val] => self * & right ; [val ref] => & self * right ; [ref ref] => Similarity :: from_isometry (self * & right . isometry , right . scaling ()) ;) ;
};
}
