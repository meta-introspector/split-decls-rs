// Generated macro for macro_2346 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2346 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2346"}
// Dependencies: {}
isometry_from_composition_impl_all ! (Mul , mul ; D ; self : Translation < T , D >, right : Rotation < T , D >, Output = Isometry < T , Rotation < T , D >, D >; [val val] => Isometry :: from_parts (self , right) ; [ref val] => Isometry :: from_parts (self . clone () , right) ; [val ref] => Isometry :: from_parts (self , right . clone ()) ; [ref ref] => Isometry :: from_parts (self . clone () , right . clone ()) ;) ;
};
}
