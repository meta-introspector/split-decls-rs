// Generated macro for macro_1778 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1778 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1778"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U4 , U1) , (U4 , U1) ; self : DualQuaternion < T >, rhs : &'b UnitDualQuaternion < T >, Output = DualQuaternion < T >; # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () . dual_quaternion () } ; 'b) ;
};
}
