// Generated macro for macro_1764 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1764 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1764"}
// Dependencies: {}
dual_quaternion_op_impl ! (Sub , sub ; (U4 , U1) , (U4 , U1) ; self : &'a DualQuaternion < T >, rhs : &'b DualQuaternion < T >, Output = DualQuaternion < T >; DualQuaternion :: from_real_and_dual (& self . real - & rhs . real , & self . dual - & rhs . dual ,) ; 'a , 'b) ;
};
}
