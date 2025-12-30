// Generated macro for macro_1831 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1831 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1831"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U4 , U1) , (U3 , U3) ; self : UnitDualQuaternion < T >, rhs : Isometry3 < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; self / UnitDualQuaternion ::< T >:: from_isometry (& rhs) ;) ;
};
}
