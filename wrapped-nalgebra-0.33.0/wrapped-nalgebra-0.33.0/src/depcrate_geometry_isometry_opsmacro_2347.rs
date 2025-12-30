// Generated macro for macro_2347 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2347 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2347"}
// Dependencies: {}
isometry_from_composition_impl_all ! (Mul , mul ; ; self : Translation < T , 3 >, right : UnitQuaternion < T >, Output = Isometry < T , UnitQuaternion < T >, 3 >; [val val] => Isometry :: from_parts (self , right) ; [ref val] => Isometry :: from_parts (self . clone () , right) ; [val ref] => Isometry :: from_parts (self , right . clone ()) ; [ref ref] => Isometry :: from_parts (self . clone () , right . clone ()) ;) ;
};
}
