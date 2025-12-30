// Generated macro for macro_1765 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1765 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1765"}
// Dependencies: {}
dual_quaternion_op_impl ! (Sub , sub ; (U4 , U1) , (U4 , U1) ; self : &'a DualQuaternion < T >, rhs : DualQuaternion < T >, Output = DualQuaternion < T >; DualQuaternion :: from_real_and_dual (& self . real - rhs . real , & self . dual - rhs . dual ,) ; 'a) ;
};
}
