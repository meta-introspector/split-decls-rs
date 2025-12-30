// Generated macro for macro_1639 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1639 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1639"}
// Dependencies: {}
quaternion_op_impl ! (Div , div ; ; self : UnitQuaternion < T >, rhs : Rotation < T , 3 >, Output = UnitQuaternion < T >; self / UnitQuaternion ::< T >:: from_rotation_matrix (& rhs) ;) ;
};
}
