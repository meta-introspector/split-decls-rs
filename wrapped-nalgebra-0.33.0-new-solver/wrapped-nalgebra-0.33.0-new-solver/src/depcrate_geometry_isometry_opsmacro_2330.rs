// Generated macro for macro_2330 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2330 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2330"}
// Dependencies: {}
isometry_binop_impl_all ! (Mul , mul ; self : Isometry < T , R , D >, right : SVector < T , D >, Output = SVector < T , D >; [val val] => self . rotation . transform_vector (& right) ; [ref val] => self . rotation . transform_vector (& right) ; [val ref] => self . rotation . transform_vector (right) ; [ref ref] => self . rotation . transform_vector (right) ;) ;
};
}
