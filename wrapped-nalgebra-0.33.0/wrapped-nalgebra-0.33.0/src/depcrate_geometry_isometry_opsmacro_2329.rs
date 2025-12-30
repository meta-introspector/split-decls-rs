// Generated macro for macro_2329 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2329 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2329"}
// Dependencies: {}
isometry_binop_impl_all ! (Mul , mul ; self : Isometry < T , R , D >, right : Point < T , D >, Output = Point < T , D >; [val val] => self . translation * self . rotation . transform_point (& right) ; [ref val] => & self . translation * self . rotation . transform_point (& right) ; [val ref] => self . translation * self . rotation . transform_point (right) ; [ref ref] => & self . translation * self . rotation . transform_point (right) ;) ;
};
}
