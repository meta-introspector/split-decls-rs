// Generated macro for macro_1794 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1794 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1794"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U4 , U1) ; self : UnitDualQuaternion < T >, rhs : &'b UnitQuaternion < T >, Output = UnitDualQuaternion < T > => U3 , U3 ; self * UnitDualQuaternion ::< T >:: new_unchecked (DualQuaternion :: from_real (rhs . clone () . into_inner ())) ; 'b) ;
};
}
