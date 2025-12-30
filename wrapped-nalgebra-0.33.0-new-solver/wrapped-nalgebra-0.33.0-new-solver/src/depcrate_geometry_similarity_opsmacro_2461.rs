// Generated macro for macro_2461 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2461 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2461"}
// Dependencies: {}
similarity_from_composition_impl_all ! (Mul , mul ; ; self : Similarity < T , UnitQuaternion < T >, 3 >, rhs : UnitQuaternion < T >, Output = Similarity < T , UnitQuaternion < T >, 3 >; [val val] => { let scaling = self . scaling () ; Similarity :: from_isometry (self . isometry * rhs , scaling) } ; [ref val] => Similarity :: from_isometry (& self . isometry * rhs , self . scaling ()) ; [val ref] => { let scaling = self . scaling () ; Similarity :: from_isometry (self . isometry * rhs , scaling) } ; [ref ref] => Similarity :: from_isometry (& self . isometry * rhs , self . scaling ()) ;) ;
};
}
