// Generated macro for macro_2462 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2462 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2462"}
// Dependencies: {}
similarity_from_composition_impl_all ! (Mul , mul ; ; self : UnitQuaternion < T >, right : Similarity < T , UnitQuaternion < T >, 3 >, Output = Similarity < T , UnitQuaternion < T >, 3 >; [val val] => & self * & right ; [ref val] => self * & right ; [val ref] => & self * right ; [ref ref] => Similarity :: from_isometry (self * & right . isometry , right . scaling ()) ;) ;
};
}
