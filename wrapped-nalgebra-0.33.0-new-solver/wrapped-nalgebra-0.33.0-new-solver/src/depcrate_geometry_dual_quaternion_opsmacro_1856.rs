// Generated macro for macro_1856 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1856 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1856"}
// Dependencies: {}
dual_quaternion_op_impl ! (AddAssign , add_assign ; (U4 , U1) , (U4 , U1) ; self : DualQuaternion < T >, rhs : DualQuaternion < T >; { self . real += rhs . real ; self . dual += rhs . dual ; } ;) ;
};
}
