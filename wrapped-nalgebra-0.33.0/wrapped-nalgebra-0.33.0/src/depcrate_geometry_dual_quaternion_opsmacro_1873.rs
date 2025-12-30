// Generated macro for macro_1873 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1873 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1873"}
// Dependencies: {}
dual_quaternion_op_impl ! (MulAssign , mul_assign ; (U4 , U1) , (U4 , U1) ; self : UnitDualQuaternion < T >, rhs : Translation3 < T >; { let res = &* self * UnitDualQuaternion :: from_parts (rhs , UnitQuaternion :: identity ()) ; self . as_mut_unchecked () . real . coords . copy_from (& res . as_ref () . real . coords) ; self . as_mut_unchecked () . dual . coords . copy_from (& res . as_ref () . dual . coords) ; } ;) ;
};
}
