// Generated macro for macro_2332 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2332 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2332"}
// Dependencies: {}
isometry_binop_impl_all ! (Mul , mul ; self : Isometry < T , R , D >, right : Translation < T , D >, Output = Isometry < T , R , D >; [val val] => & self * & right ; [ref val] => self * & right ; [val ref] => & self * right ; [ref ref] => { # [allow (clippy :: suspicious_arithmetic_impl)] let new_tr = & self . translation . vector + self . rotation . transform_vector (& right . vector) ; Isometry :: from_parts (Translation :: from (new_tr) , self . rotation . clone ()) } ;) ;
};
}
