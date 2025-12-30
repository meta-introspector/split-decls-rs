// Generated macro for macro_1870 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1870 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1870"}
// Dependencies: {}
dual_quaternion_op_impl ! (MulAssign , mul_assign ; (U4 , U1) , (U4 , U1) ; self : UnitDualQuaternion < T >, rhs : &'b UnitQuaternion < T >; * self *= rhs . clone () ; 'b) ;
};
}
