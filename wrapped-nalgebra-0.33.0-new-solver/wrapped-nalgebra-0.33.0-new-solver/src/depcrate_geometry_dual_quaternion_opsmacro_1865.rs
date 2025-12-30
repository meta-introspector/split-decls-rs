// Generated macro for macro_1865 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1865 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1865"}
// Dependencies: {}
dual_quaternion_op_impl ! (MulAssign , mul_assign ; (U4 , U1) , (U4 , U1) ; self : UnitDualQuaternion < T >, rhs : &'b UnitDualQuaternion < T >; { let res = &* self * rhs ; self . as_mut_unchecked () . real . coords . copy_from (& res . as_ref () . real . coords) ; self . as_mut_unchecked () . dual . coords . copy_from (& res . as_ref () . dual . coords) ; } ; 'b) ;
};
}
