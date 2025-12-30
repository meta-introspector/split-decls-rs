// Generated macro for macro_1836 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1836 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1836"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U3 , U1) , (U4 , U1) ; self : &'a Isometry3 < T >, rhs : &'b UnitDualQuaternion < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; UnitDualQuaternion ::< T >:: from_isometry (self) / rhs ; 'a , 'b) ;
};
}
