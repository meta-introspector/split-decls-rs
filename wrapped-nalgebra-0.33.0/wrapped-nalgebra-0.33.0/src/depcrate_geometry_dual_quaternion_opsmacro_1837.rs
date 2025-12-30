// Generated macro for macro_1837 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1837 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1837"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U3 , U1) , (U4 , U1) ; self : &'a Isometry3 < T >, rhs : UnitDualQuaternion < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; UnitDualQuaternion ::< T >:: from_isometry (self) / rhs ; 'a) ;
};
}
