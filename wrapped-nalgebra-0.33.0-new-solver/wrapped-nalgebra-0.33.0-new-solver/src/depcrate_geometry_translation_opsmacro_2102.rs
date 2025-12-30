// Generated macro for macro_2102 (macro)
macro_rules! Depcrate_geometry_translation_opsmacro_2102 {
() => {
// Module: crate::geometry::translation_ops
// Provides: {"macro_2102"}
// Dependencies: {}
add_sub_assign_impl ! (DivAssign , div_assign , ClosedSubAssign ; const D ; self : Translation < T , D >, right : &'b Translation < T , D >; # [allow (clippy :: suspicious_op_assign_impl)] { self . vector -= & right . vector } ; 'b) ;
};
}
