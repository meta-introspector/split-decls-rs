// Generated macro for macro_1642 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1642 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1642"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : Rotation < T , 3 >, rhs : &'b UnitQuaternion < T >, Output = UnitQuaternion < T >; UnitQuaternion ::< T >:: from_rotation_matrix (& self) * rhs ; 'b) ;
};
}
