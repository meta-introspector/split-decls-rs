// Generated macro for macro_1652 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1652 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1652"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : &'a UnitQuaternion < T >, rhs : &'b Point3 < T >, Output = Point3 < T >; Point3 :: from (self * & rhs . coords) ; 'a , 'b) ;
};
}
