// Generated macro for macro_1804 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1804 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1804"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U4 , U1) , (U4 , U1) ; self : &'a UnitQuaternion < T >, rhs : &'b UnitDualQuaternion < T >, Output = UnitDualQuaternion < T > => U1 , U4 ; # [allow (clippy :: suspicious_arithmetic_impl)] { UnitDualQuaternion ::< T >:: new_unchecked (DualQuaternion :: from_real (self . clone () . into_inner ())) * rhs . inverse () } ; 'a , 'b) ;
};
}
