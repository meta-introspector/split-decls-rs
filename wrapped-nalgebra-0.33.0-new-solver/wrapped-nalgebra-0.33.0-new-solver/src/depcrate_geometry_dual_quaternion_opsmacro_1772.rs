// Generated macro for macro_1772 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1772 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1772"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U4 , U1) ; self : &'a DualQuaternion < T >, rhs : &'b UnitDualQuaternion < T >, Output = DualQuaternion < T >; self * rhs . dual_quaternion () ; 'a , 'b) ;
};
}
