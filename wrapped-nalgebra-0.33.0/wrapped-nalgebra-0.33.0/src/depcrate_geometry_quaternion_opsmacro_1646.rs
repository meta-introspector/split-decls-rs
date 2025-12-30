// Generated macro for macro_1646 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1646 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1646"}
// Dependencies: {}
quaternion_op_impl ! (Div , div ; ; self : Rotation < T , 3 >, rhs : &'b UnitQuaternion < T >, Output = UnitQuaternion < T >; UnitQuaternion ::< T >:: from_rotation_matrix (& self) / rhs ; 'b) ;
};
}
