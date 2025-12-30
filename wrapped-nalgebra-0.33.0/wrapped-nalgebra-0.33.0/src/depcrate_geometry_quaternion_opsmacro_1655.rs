// Generated macro for macro_1655 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1655 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1655"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : UnitQuaternion < T >, rhs : Point3 < T >, Output = Point3 < T >; Point3 :: from (self * rhs . coords) ;) ;
};
}
