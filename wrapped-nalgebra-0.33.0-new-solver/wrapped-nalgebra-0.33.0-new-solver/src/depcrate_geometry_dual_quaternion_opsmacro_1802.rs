// Generated macro for macro_1802 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1802 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1802"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U4 , U1) , (U4 , U1) ; self : UnitDualQuaternion < T >, rhs : &'b UnitQuaternion < T >, Output = UnitDualQuaternion < T > => U3 , U3 ; # [allow (clippy :: suspicious_arithmetic_impl)] { self * UnitDualQuaternion ::< T >:: from_rotation (rhs . inverse ()) } ; 'b) ;
};
}
