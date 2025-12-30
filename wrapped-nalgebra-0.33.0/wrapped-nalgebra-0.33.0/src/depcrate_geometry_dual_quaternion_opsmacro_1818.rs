// Generated macro for macro_1818 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1818 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1818"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U3 , U1) , (U4 , U1) ; self : Translation3 < T >, rhs : &'b UnitDualQuaternion < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; UnitDualQuaternion ::< T >:: from_parts (self , UnitQuaternion :: identity ()) * rhs ; 'b) ;
};
}
