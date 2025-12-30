// Generated macro for macro_1640 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1640 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1640"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : &'a Rotation < T , 3 >, rhs : &'b UnitQuaternion < T >, Output = UnitQuaternion < T >; UnitQuaternion ::< T >:: from_rotation_matrix (self) * rhs ; 'a , 'b) ;
};
}
