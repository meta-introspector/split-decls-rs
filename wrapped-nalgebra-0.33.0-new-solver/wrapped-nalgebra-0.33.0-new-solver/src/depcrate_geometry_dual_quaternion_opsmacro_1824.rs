// Generated macro for macro_1824 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1824 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1824"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U3 , U1) ; self : &'a UnitDualQuaternion < T >, rhs : &'b Isometry3 < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; self * UnitDualQuaternion ::< T >:: from_isometry (rhs) ; 'a , 'b) ;
};
}
