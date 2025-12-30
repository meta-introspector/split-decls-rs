// Generated macro for macro_2465 (macro)
macro_rules! Depcrate_geometry_similarity_opsmacro_2465 {
() => {
// Module: crate::geometry::similarity_ops
// Provides: {"macro_2465"}
// Dependencies: {}
similarity_from_composition_impl_all ! (Mul , mul ; ; self : Similarity < T , UnitComplex < T >, 2 >, rhs : UnitComplex < T >, Output = Similarity < T , UnitComplex < T >, 2 >; [val val] => { let scaling = self . scaling () ; Similarity :: from_isometry (self . isometry * rhs , scaling) } ; [ref val] => Similarity :: from_isometry (& self . isometry * rhs , self . scaling ()) ; [val ref] => { let scaling = self . scaling () ; Similarity :: from_isometry (self . isometry * rhs , scaling) } ; [ref ref] => Similarity :: from_isometry (& self . isometry * rhs , self . scaling ()) ;) ;
};
}
