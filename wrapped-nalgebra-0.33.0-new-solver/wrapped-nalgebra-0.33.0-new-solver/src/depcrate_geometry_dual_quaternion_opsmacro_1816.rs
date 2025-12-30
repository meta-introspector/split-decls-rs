// Generated macro for macro_1816 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1816 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1816"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U3 , U1) , (U4 , U1) ; self : &'b Translation3 < T >, rhs : &'a UnitDualQuaternion < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; UnitDualQuaternion ::< T >:: from_parts (self . clone () , UnitQuaternion :: identity ()) * rhs ; 'a , 'b) ;
};
}
