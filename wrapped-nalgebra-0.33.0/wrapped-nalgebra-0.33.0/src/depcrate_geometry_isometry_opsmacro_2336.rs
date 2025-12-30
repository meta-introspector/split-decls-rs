// Generated macro for macro_2336 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2336 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2336"}
// Dependencies: {}
isometry_from_composition_impl_all ! (Mul , mul ; D ; self : Rotation < T , D >, right : Translation < T , D >, Output = Isometry < T , Rotation < T , D >, D >; [val val] => Isometry :: from_parts (Translation :: from (& self * right . vector) , self) ; [ref val] => Isometry :: from_parts (Translation :: from (self * right . vector) , self . clone ()) ; [val ref] => Isometry :: from_parts (Translation :: from (& self * & right . vector) , self) ; [ref ref] => Isometry :: from_parts (Translation :: from (self * & right . vector) , self . clone ()) ;) ;
};
}
