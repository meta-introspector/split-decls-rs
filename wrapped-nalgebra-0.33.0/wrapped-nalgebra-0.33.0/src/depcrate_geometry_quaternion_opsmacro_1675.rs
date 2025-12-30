// Generated macro for macro_1675 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1675 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1675"}
// Dependencies: {}
quaternion_op_impl ! (DivAssign , div_assign ; self : UnitQuaternion < T >, rhs : &'b UnitQuaternion < T >; { let res = &* self / rhs ; self . as_mut_unchecked () . coords . copy_from (& res . as_ref () . coords) ; } ; 'b) ;
};
}
