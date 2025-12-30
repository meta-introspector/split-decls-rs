// Generated macro for macro_1638 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1638 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1638"}
// Dependencies: {}
quaternion_op_impl ! (Div , div ; ; self : UnitQuaternion < T >, rhs : &'b Rotation < T , 3 >, Output = UnitQuaternion < T >; self / UnitQuaternion ::< T >:: from_rotation_matrix (rhs) ; 'b) ;
};
}
