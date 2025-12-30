// Generated macro for macro_1830 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1830 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1830"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U4 , U1) , (U3 , U3) ; self : UnitDualQuaternion < T >, rhs : &'b Isometry3 < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; self / UnitDualQuaternion ::< T >:: from_isometry (rhs) ; 'b) ;
};
}
