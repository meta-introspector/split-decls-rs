// Generated macro for macro_1614 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1614 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1614"}
// Dependencies: {}
quaternion_op_impl ! (Add , add ; ; self : Quaternion < T >, rhs : &'b Quaternion < T >, Output = Quaternion < T >; Quaternion :: from (self . coords + & rhs . coords) ; 'b) ;
};
}
