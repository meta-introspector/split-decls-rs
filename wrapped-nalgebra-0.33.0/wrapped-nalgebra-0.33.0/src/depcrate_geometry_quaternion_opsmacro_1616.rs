// Generated macro for macro_1616 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1616 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1616"}
// Dependencies: {}
quaternion_op_impl ! (Sub , sub ; ; self : &'a Quaternion < T >, rhs : &'b Quaternion < T >, Output = Quaternion < T >; Quaternion :: from (& self . coords - & rhs . coords) ; 'a , 'b) ;
};
}
