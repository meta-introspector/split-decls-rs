// Generated macro for macro_1823 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1823 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1823"}
// Dependencies: {}
dual_quaternion_op_impl ! (Div , div ; (U3 , U1) , (U4 , U1) ; self : Translation3 < T >, rhs : UnitDualQuaternion < T >, Output = UnitDualQuaternion < T > => U3 , U1 ; UnitDualQuaternion ::< T >:: from_parts (self , UnitQuaternion :: identity ()) / rhs ;) ;
};
}
