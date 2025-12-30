// Generated macro for macro_2321 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2321 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2321"}
// Dependencies: {}
isometry_binop_assign_impl_all ! (MulAssign , mul_assign ; self : Isometry < T , R , D >, rhs : Isometry < T , R , D >; [val] => * self *= & rhs ; [ref] => { let shift = self . rotation . transform_vector (& rhs . translation . vector) ; self . translation . vector += shift ; self . rotation *= rhs . rotation . clone () ; } ;) ;
};
}
