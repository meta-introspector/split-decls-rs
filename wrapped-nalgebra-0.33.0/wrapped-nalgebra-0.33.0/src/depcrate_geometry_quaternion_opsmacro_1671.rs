// Generated macro for macro_1671 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1671 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1671"}
// Dependencies: {}
quaternion_op_impl ! (MulAssign , mul_assign ; self : Quaternion < T >, rhs : &'b Quaternion < T >; { let res = &* self * rhs ; self . coords . copy_from (& res . coords) ; } ; 'b) ;
};
}
