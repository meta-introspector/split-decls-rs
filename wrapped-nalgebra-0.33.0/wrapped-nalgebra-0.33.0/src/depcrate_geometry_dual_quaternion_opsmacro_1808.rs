// Generated macro for macro_1808 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1808 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1808"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U3 , U1) ; self : &'a UnitDualQuaternion < T >, rhs : &'b Translation3 < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; self * UnitDualQuaternion ::< T >:: from_parts (rhs . clone () , UnitQuaternion :: identity ()) ; 'a , 'b) ;
};
}
