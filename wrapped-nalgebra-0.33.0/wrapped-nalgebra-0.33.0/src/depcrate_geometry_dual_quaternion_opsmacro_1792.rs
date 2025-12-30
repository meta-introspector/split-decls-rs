// Generated macro for macro_1792 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1792 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1792"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U4 , U1) ; self : &'a UnitDualQuaternion < T >, rhs : &'b UnitQuaternion < T >, Output = UnitDualQuaternion < T > => U1 , U4 ; self * UnitDualQuaternion ::< T >:: new_unchecked (DualQuaternion :: from_real (rhs . clone () . into_inner ())) ; 'a , 'b) ;
};
}
