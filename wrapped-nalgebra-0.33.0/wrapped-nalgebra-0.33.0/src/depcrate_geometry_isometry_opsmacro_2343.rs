// Generated macro for macro_2343 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2343 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2343"}
// Dependencies: {}
isometry_from_composition_impl_all ! (Mul , mul ; ; self : UnitQuaternion < T >, right : Isometry < T , UnitQuaternion < T >, 3 >, Output = Isometry < T , UnitQuaternion < T >, 3 >; [val val] => & self * & right ; [ref val] => self * & right ; [val ref] => & self * right ; [ref ref] => { let shift = self * & right . translation . vector ; Isometry :: from_parts (Translation :: from (shift) , self * & right . rotation) } ;) ;
};
}
