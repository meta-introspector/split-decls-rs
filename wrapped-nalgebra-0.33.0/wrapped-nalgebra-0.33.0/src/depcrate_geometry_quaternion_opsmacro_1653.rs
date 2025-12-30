// Generated macro for macro_1653 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1653 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1653"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : &'a UnitQuaternion < T >, rhs : Point3 < T >, Output = Point3 < T >; Point3 :: from (self * rhs . coords) ; 'a) ;
};
}
