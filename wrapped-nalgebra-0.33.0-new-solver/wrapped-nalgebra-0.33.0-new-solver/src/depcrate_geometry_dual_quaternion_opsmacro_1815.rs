// Generated macro for macro_1815 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1815 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1815"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U4 , U1) , (U3 , U3) ; self : UnitDualQuaternion < T >, rhs : Translation3 < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; # [allow (clippy :: suspicious_arithmetic_impl)] { self * UnitDualQuaternion ::< T >:: from_parts (rhs . inverse () , UnitQuaternion :: identity ()) } ;) ;
};
}
