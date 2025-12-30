// Generated macro for macro_1776 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1776 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1776"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U4 , U1) , (U4 , U1) ; self : &'a DualQuaternion < T >, rhs : &'b UnitDualQuaternion < T >, Output = DualQuaternion < T >; # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () . dual_quaternion () } ; 'a , 'b) ;
};
}
