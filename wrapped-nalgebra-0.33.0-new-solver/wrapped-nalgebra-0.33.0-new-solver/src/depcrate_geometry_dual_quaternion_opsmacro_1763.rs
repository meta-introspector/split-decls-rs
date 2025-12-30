// Generated macro for macro_1763 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1763 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1763"}
// Dependencies: {}
dual_quaternion_op_impl ! (Add , add ; (U4 , U1) , (U4 , U1) ; self : DualQuaternion < T >, rhs : DualQuaternion < T >, Output = DualQuaternion < T >; DualQuaternion :: from_real_and_dual (self . real + rhs . real , self . dual + rhs . dual ,) ;) ;
};
}
