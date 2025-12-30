// Generated macro for macro_2318 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2318 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2318"}
// Dependencies: {}
isometry_binop_impl_all ! (Mul , mul ; self : Isometry < T , R , D >, rhs : Isometry < T , R , D >, Output = Isometry < T , R , D >; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => { let shift = self . rotation . transform_vector (& rhs . translation . vector) ; # [allow (clippy :: suspicious_arithmetic_impl)] Isometry :: from_parts (Translation :: from (& self . translation . vector + shift) , self . rotation . clone () * rhs . rotation . clone ()) } ;) ;
};
}
