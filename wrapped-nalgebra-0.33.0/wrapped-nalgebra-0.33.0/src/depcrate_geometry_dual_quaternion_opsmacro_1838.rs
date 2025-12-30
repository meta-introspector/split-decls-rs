// Generated macro for macro_1838 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1838 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1838"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U3 , U1) , (U4 , U1) ; self : Isometry3 < T >, rhs : &'b UnitDualQuaternion < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; UnitDualQuaternion ::< T >:: from_isometry (& self) / rhs ; 'b) ;
};
}
