// Generated macro for macro_1766 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1766 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1766"}
// Dependencies: {}
dual_quaternion_op_impl ! (Sub , sub ; (U4 , U1) , (U4 , U1) ; self : DualQuaternion < T >, rhs : &'b DualQuaternion < T >, Output = DualQuaternion < T >; DualQuaternion :: from_real_and_dual (self . real - & rhs . real , self . dual - & rhs . dual ,) ; 'b) ;
};
}
