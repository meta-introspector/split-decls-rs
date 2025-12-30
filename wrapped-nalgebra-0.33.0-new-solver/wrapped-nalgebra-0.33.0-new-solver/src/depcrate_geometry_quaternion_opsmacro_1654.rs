// Generated macro for macro_1654 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1654 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1654"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : UnitQuaternion < T >, rhs : &'b Point3 < T >, Output = Point3 < T >; Point3 :: from (self * & rhs . coords) ; 'b) ;
};
}
