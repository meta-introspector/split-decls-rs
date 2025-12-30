// Generated macro for macro_1879 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1879 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1879"}
// Dependencies: {}
dual_quaternion_op_impl ! (DivAssign , div_assign ; (U4 , U1) , (U3 , U1) ; self : UnitDualQuaternion < T >, rhs : &'b Isometry3 < T > => U3 , U1 ; { let res = &* self / rhs ; self . as_mut_unchecked () . real . coords . copy_from (& res . as_ref () . real . coords) ; self . as_mut_unchecked () . dual . coords . copy_from (& res . as_ref () . dual . coords) ; } ; 'b) ;
};
}
