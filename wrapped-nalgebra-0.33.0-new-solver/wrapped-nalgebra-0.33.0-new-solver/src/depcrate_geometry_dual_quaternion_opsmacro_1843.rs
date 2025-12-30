// Generated macro for macro_1843 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1843 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1843"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U3 , U1) for SB : Storage < T , U3 > ; self : UnitDualQuaternion < T >, rhs : Vector < T , U3 , SB >, Output = Vector3 < T > => U3 , U1 ; & self * & rhs ;) ;
};
}
