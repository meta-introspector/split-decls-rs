// Generated macro for macro_1641 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1641 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1641"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : &'a Rotation < T , 3 >, rhs : UnitQuaternion < T >, Output = UnitQuaternion < T >; UnitQuaternion ::< T >:: from_rotation_matrix (self) * rhs ; 'a) ;
};
}
