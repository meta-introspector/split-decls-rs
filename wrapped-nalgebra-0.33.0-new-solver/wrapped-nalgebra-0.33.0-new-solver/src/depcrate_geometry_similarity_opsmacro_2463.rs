// Generated macro for macro_2463 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2463 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2463"}
// Dependencies: {}
similarity_from_composition_impl_all ! (Div , div ; ; self : Similarity < T , UnitQuaternion < T >, 3 >, rhs : UnitQuaternion < T >, Output = Similarity < T , UnitQuaternion < T >, 3 >; [val val] => { let scaling = self . scaling () ; Similarity :: from_isometry (self . isometry / rhs , scaling) } ; [ref val] => Similarity :: from_isometry (& self . isometry / rhs , self . scaling ()) ; [val ref] => { let scaling = self . scaling () ; Similarity :: from_isometry (self . isometry / rhs , scaling) } ; [ref ref] => Similarity :: from_isometry (& self . isometry / rhs , self . scaling ()) ;) ;
};
}
