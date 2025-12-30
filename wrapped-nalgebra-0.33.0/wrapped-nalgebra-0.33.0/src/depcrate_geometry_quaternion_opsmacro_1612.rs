// Generated macro for macro_1612 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1612 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1612"}
// Dependencies: {}
quaternion_op_impl ! (Add , add ; ; self : &'a Quaternion < T >, rhs : &'b Quaternion < T >, Output = Quaternion < T >; Quaternion :: from (& self . coords + & rhs . coords) ; 'a , 'b) ;
};
}
