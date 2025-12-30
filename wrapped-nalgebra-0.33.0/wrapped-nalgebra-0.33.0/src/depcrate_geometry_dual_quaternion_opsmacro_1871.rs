// Generated macro for macro_1871 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1871 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1871"}
// Dependencies: {}
dual_quaternion_op_impl ! (DivAssign , div_assign ; (U4 , U1) , (U4 , U1) ; self : UnitDualQuaternion < T >, rhs : &'b UnitQuaternion < T >; # [allow (clippy :: suspicious_op_assign_impl)] { let res = &* self * UnitDualQuaternion :: from_rotation (rhs . inverse ()) ; self . as_mut_unchecked () . real . coords . copy_from (& res . as_ref () . real . coords) ; self . as_mut_unchecked () . dual . coords . copy_from (& res . as_ref () . dual . coords) ; } ; 'b) ;
};
}
