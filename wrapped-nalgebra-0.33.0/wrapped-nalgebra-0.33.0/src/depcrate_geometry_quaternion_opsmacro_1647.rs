// Generated macro for macro_1647 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1647 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1647"}
// Dependencies: {}
quaternion_op_impl ! (Div , div ; ; self : Rotation < T , 3 >, rhs : UnitQuaternion < T >, Output = UnitQuaternion < T >; UnitQuaternion ::< T >:: from_rotation_matrix (& self) / rhs ;) ;
};
}
