// Generated macro for macro_1869 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1869 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1869"}
// Dependencies: {}
dual_quaternion_op_impl ! (MulAssign , mul_assign ; (U4 , U1) , (U4 , U1) ; self : UnitDualQuaternion < T >, rhs : UnitQuaternion < T >; { let res = &* self * UnitDualQuaternion :: from_rotation (rhs) ; self . as_mut_unchecked () . real . coords . copy_from (& res . as_ref () . real . coords) ; self . as_mut_unchecked () . dual . coords . copy_from (& res . as_ref () . dual . coords) ; } ;) ;
};
}
