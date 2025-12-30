// Generated macro for macro_1875 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1875 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1875"}
// Dependencies: {}
dual_quaternion_op_impl ! (DivAssign , div_assign ; (U4 , U1) , (U4 , U1) ; self : UnitDualQuaternion < T >, rhs : &'b Translation3 < T >; # [allow (clippy :: suspicious_op_assign_impl)] { let res = &* self * UnitDualQuaternion :: from_parts (rhs . inverse () , UnitQuaternion :: identity ()) ; self . as_mut_unchecked () . real . coords . copy_from (& res . as_ref () . real . coords) ; self . as_mut_unchecked () . dual . coords . copy_from (& res . as_ref () . dual . coords) ; } ; 'b) ;
};
}
