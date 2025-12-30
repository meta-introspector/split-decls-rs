// Generated macro for macro_1644 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1644 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1644"}
// Dependencies: {}
quaternion_op_impl ! (Div , div ; ; self : &'a Rotation < T , 3 >, rhs : &'b UnitQuaternion < T >, Output = UnitQuaternion < T >; UnitQuaternion ::< T >:: from_rotation_matrix (self) / rhs ; 'a , 'b) ;
};
}
