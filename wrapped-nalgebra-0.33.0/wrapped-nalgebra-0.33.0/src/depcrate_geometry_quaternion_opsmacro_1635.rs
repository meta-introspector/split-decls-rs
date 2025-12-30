// Generated macro for macro_1635 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1635 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1635"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : UnitQuaternion < T >, rhs : Rotation < T , 3 >, Output = UnitQuaternion < T >; self * UnitQuaternion ::< T >:: from_rotation_matrix (& rhs) ;) ;
};
}
