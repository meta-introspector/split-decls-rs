// Generated macro for macro_1613 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1613 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1613"}
// Dependencies: {}
quaternion_op_impl ! (Add , add ; ; self : &'a Quaternion < T >, rhs : Quaternion < T >, Output = Quaternion < T >; Quaternion :: from (& self . coords + rhs . coords) ; 'a) ;
};
}
