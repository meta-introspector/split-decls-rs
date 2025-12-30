// Generated macro for macro_1632 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1632 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1632"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : &'a UnitQuaternion < T >, rhs : &'b Rotation < T , 3 >, Output = UnitQuaternion < T >; self * UnitQuaternion ::< T >:: from_rotation_matrix (rhs) ; 'a , 'b) ;
};
}
