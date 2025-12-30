// Generated macro for macro_1628 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1628 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1628"}
// Dependencies: {}
quaternion_op_impl ! (Div , div ; ; self : &'a UnitQuaternion < T >, rhs : &'b UnitQuaternion < T >, Output = UnitQuaternion < T >; # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; 'a , 'b) ;
};
}
