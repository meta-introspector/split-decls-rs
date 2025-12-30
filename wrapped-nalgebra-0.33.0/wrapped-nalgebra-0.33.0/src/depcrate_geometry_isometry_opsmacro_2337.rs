// Generated macro for macro_2337 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2337 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2337"}
// Dependencies: {}
isometry_from_composition_impl_all ! (Mul , mul ; ; self : UnitQuaternion < T >, right : Translation < T , 3 >, Output = Isometry < T , UnitQuaternion < T >, 3 >; [val val] => Isometry :: from_parts (Translation :: from (& self * right . vector) , self) ; [ref val] => Isometry :: from_parts (Translation :: from (self * right . vector) , self . clone ()) ; [val ref] => Isometry :: from_parts (Translation :: from (& self * & right . vector) , self) ; [ref ref] => Isometry :: from_parts (Translation :: from (self * & right . vector) , self . clone ()) ;) ;
};
}
