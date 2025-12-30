// Generated macro for macro_2331 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2331 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2331"}
// Dependencies: {}
isometry_binop_impl_all ! (Mul , mul ; self : Isometry < T , R , D >, right : Unit < SVector < T , D >>, Output = Unit < SVector < T , D >>; [val val] => Unit :: new_unchecked (self . rotation . transform_vector (right . as_ref ())) ; [ref val] => Unit :: new_unchecked (self . rotation . transform_vector (right . as_ref ())) ; [val ref] => Unit :: new_unchecked (self . rotation . transform_vector (right . as_ref ())) ; [ref ref] => Unit :: new_unchecked (self . rotation . transform_vector (right . as_ref ())) ;) ;
};
}
