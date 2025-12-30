// Generated macro for macro_1636 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1636 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1636"}
// Dependencies: {}
quaternion_op_impl ! (Div , div ; ; self : &'a UnitQuaternion < T >, rhs : &'b Rotation < T , 3 >, Output = UnitQuaternion < T >; self / UnitQuaternion ::< T >:: from_rotation_matrix (rhs) ; 'a , 'b) ;
};
}
