// Generated macro for macro_1874 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1874 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1874"}
// Dependencies: {}
dual_quaternion_op_impl ! (MulAssign , mul_assign ; (U4 , U1) , (U4 , U1) ; self : UnitDualQuaternion < T >, rhs : &'b Translation3 < T >; * self *= rhs . clone () ; 'b) ;
};
}
