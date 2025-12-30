// Generated macro for macro_1679 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1679 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1679"}
// Dependencies: {}
quaternion_op_impl ! (DivAssign , div_assign ; self : UnitQuaternion < T >, rhs : &'b Rotation < T , 3 >; { let res = &* self / rhs ; self . as_mut_unchecked () . coords . copy_from (& res . as_ref () . coords) ; } ; 'b) ;
};
}
