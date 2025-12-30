// Generated macro for macro_2339 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2339 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2339"}
// Dependencies: {}
isometry_from_composition_impl_all ! (Mul , mul ; D ; self : Rotation < T , D >, right : Isometry < T , Rotation < T , D >, D >, Output = Isometry < T , Rotation < T , D >, D >; [val val] => & self * & right ; [ref val] => self * & right ; [val ref] => & self * right ; [ref ref] => { let shift = self * & right . translation . vector ; Isometry :: from_parts (Translation :: from (shift) , self * & right . rotation) } ;) ;
};
}
