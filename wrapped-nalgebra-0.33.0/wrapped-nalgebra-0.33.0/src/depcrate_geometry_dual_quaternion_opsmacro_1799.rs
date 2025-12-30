// Generated macro for macro_1799 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1799 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1799"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U4 , U1) ; self : UnitQuaternion < T >, rhs : UnitDualQuaternion < T >, Output = UnitDualQuaternion < T > => U3 , U3 ; UnitDualQuaternion ::< T >:: new_unchecked (DualQuaternion :: from_real (self . into_inner ())) * rhs ;) ;
};
}
