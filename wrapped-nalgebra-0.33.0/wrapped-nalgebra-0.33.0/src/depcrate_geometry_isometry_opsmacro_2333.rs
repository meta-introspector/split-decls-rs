// Generated macro for macro_2333 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2333 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2333"}
// Dependencies: {}
isometry_binop_impl_all ! (Mul , mul ; self : Translation < T , D >, right : Isometry < T , R , D >, Output = Isometry < T , R , D >; [val val] => Isometry :: from_parts (self * right . translation , right . rotation) ; [ref val] => Isometry :: from_parts (self * & right . translation , right . rotation) ; [val ref] => Isometry :: from_parts (self * & right . translation , right . rotation . clone ()) ; [ref ref] => Isometry :: from_parts (self * & right . translation , right . rotation . clone ()) ;) ;
};
}
