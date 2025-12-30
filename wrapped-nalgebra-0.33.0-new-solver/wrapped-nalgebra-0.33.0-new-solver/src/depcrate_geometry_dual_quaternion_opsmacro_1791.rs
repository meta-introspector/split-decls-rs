// Generated macro for macro_1791 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1791 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1791"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U4 , U1) ; self : UnitDualQuaternion < T >, rhs : DualQuaternion < T >, Output = DualQuaternion < T > => U3 , U3 ; self . dual_quaternion () * rhs ;) ;
};
}
