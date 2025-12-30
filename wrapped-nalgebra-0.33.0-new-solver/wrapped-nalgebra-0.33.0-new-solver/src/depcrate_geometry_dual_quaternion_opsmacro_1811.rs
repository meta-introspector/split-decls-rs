// Generated macro for macro_1811 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1811 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1811"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U3 , U3) ; self : UnitDualQuaternion < T >, rhs : Translation3 < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; self * UnitDualQuaternion ::< T >:: from_parts (rhs , UnitQuaternion :: identity ()) ;) ;
};
}
