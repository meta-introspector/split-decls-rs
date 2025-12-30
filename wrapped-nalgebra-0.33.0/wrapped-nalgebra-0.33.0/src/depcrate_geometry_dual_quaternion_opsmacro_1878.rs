// Generated macro for macro_1878 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1878 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1878"}
// Dependencies: {}
dual_quaternion_op_impl ! (MulAssign , mul_assign ; (U4 , U1) , (U3 , U1) ; self : UnitDualQuaternion < T >, rhs : Isometry3 < T > => U3 , U1 ; * self *= & rhs ;) ;
};
}
