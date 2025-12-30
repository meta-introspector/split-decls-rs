// Generated macro for macro_1863 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1863 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1863"}
// Dependencies: {}
dual_quaternion_op_impl ! (DivAssign , div_assign ; (U4 , U1) , (U4 , U1) ; self : DualQuaternion < T >, rhs : &'b UnitDualQuaternion < T >; { let res = &* self / rhs ; self . real . coords . copy_from (& res . real . coords) ; self . dual . coords . copy_from (& res . dual . coords) ; } ; 'b) ;
};
}
