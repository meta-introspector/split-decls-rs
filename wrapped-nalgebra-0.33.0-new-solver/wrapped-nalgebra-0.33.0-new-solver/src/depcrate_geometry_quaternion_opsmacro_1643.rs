// Generated macro for macro_1643 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1643 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1643"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : Rotation < T , 3 >, rhs : UnitQuaternion < T >, Output = UnitQuaternion < T >; UnitQuaternion ::< T >:: from_rotation_matrix (& self) * rhs ;) ;
};
}
