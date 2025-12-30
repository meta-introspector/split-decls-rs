// Generated macro for macro_1857 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1857 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1857"}
// Dependencies: {}
dual_quaternion_op_impl ! (SubAssign , sub_assign ; (U4 , U1) , (U4 , U1) ; self : DualQuaternion < T >, rhs : &'b DualQuaternion < T >; { self . real -= & rhs . real ; self . dual -= & rhs . dual ; } ; 'b) ;
};
}
