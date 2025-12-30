// Generated macro for macro_1775 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1775 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1775"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U4 , U1) ; self : DualQuaternion < T >, rhs : UnitDualQuaternion < T >, Output = DualQuaternion < T >; self * rhs . dual_quaternion () ;) ;
};
}
