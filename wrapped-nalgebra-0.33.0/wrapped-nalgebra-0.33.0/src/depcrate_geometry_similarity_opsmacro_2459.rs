// Generated macro for macro_2459 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2459 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2459"}
// Dependencies: {}
similarity_from_composition_impl_all ! (Div , div ; D ; self : Similarity < T , Rotation < T , D >, D >, rhs : Rotation < T , D >, Output = Similarity < T , Rotation < T , D >, D >; [val val] => { let scaling = self . scaling () ; Similarity :: from_isometry (self . isometry / rhs , scaling) } ; [ref val] => Similarity :: from_isometry (& self . isometry / rhs , self . scaling ()) ; [val ref] => { let scaling = self . scaling () ; Similarity :: from_isometry (self . isometry / rhs , scaling) } ; [ref ref] => Similarity :: from_isometry (& self . isometry / rhs , self . scaling ()) ;) ;
};
}
