// Generated macro for macro_1806 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1806 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1806"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U4 , U1) , (U4 , U1) ; self : UnitQuaternion < T >, rhs : &'b UnitDualQuaternion < T >, Output = UnitDualQuaternion < T > => U3 , U3 ; # [allow (clippy :: suspicious_arithmetic_impl)] { UnitDualQuaternion ::< T >:: new_unchecked (DualQuaternion :: from_real (self . into_inner ())) * rhs . inverse () } ; 'b) ;
};
}
