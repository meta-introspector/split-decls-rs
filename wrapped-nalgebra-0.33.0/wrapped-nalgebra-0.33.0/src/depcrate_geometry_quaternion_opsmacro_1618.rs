// Generated macro for macro_1618 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1618 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1618"}
// Dependencies: {}
quaternion_op_impl ! (Sub , sub ; ; self : Quaternion < T >, rhs : &'b Quaternion < T >, Output = Quaternion < T >; Quaternion :: from (self . coords - & rhs . coords) ; 'b) ;
};
}
