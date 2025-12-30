// Generated macro for macro_1768 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1768 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1768"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U4 , U1) ; self : &'a DualQuaternion < T >, rhs : &'b DualQuaternion < T >, Output = DualQuaternion < T >; DualQuaternion :: from_real_and_dual (& self . real * & rhs . real , & self . real * & rhs . dual + & self . dual * & rhs . real ,) ; 'a , 'b) ;
};
}
