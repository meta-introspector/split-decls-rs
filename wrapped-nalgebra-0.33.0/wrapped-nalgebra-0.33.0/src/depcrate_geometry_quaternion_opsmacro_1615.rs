// Generated macro for macro_1615 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1615 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1615"}
// Dependencies: {}
quaternion_op_impl ! (Add , add ; ; self : Quaternion < T >, rhs : Quaternion < T >, Output = Quaternion < T >; Quaternion :: from (self . coords + rhs . coords) ;) ;
};
}
