// Generated macro for macro_1813 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1813 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1813"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U4 , U1) , (U3 , U3) ; self : &'a UnitDualQuaternion < T >, rhs : Translation3 < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; # [allow (clippy :: suspicious_arithmetic_impl)] { self * UnitDualQuaternion ::< T >:: from_parts (rhs . inverse () , UnitQuaternion :: identity ()) } ; 'a) ;
};
}
