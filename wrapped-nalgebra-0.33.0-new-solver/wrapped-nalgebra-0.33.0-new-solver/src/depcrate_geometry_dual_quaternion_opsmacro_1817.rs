// Generated macro for macro_1817 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1817 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1817"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U3 , U1) , (U4 , U1) ; self : &'a Translation3 < T >, rhs : UnitDualQuaternion < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; UnitDualQuaternion ::< T >:: from_parts (self . clone () , UnitQuaternion :: identity ()) * rhs ; 'a) ;
};
}
