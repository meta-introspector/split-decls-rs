// Generated macro for macro_1828 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1828 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1828"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U4 , U1) , (U3 , U1) ; self : &'a UnitDualQuaternion < T >, rhs : &'b Isometry3 < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; self / UnitDualQuaternion ::< T >:: from_isometry (rhs) ; 'a , 'b) ;
};
}
