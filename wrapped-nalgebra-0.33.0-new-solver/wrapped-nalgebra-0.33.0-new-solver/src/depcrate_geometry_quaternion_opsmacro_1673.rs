// Generated macro for macro_1673 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1673 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1673"}
// Dependencies: {}
quaternion_op_impl ! (MulAssign , mul_assign ; self : UnitQuaternion < T >, rhs : &'b UnitQuaternion < T >; { let res = &* self * rhs ; self . as_mut_unchecked () . coords . copy_from (& res . as_ref () . coords) ; } ; 'b) ;
};
}
