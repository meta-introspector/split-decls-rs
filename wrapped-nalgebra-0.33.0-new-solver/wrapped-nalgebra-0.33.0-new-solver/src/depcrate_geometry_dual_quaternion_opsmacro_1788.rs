// Generated macro for macro_1788 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1788 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1788"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U4 , U1) ; self : &'a UnitDualQuaternion < T >, rhs : &'b DualQuaternion < T >, Output = DualQuaternion < T > => U1 , U4 ; self . dual_quaternion () * rhs ; 'a , 'b) ;
};
}
