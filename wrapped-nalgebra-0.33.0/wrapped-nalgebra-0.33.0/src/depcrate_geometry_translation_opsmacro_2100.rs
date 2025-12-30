// Generated macro for macro_2100 (macro)
macro_rules! Depcrate_geometry_translation_opsmacro_2100 {
() => {
// Module: crate::geometry::translation_ops
// Provides: {"macro_2100"}
// Dependencies: {}
add_sub_assign_impl ! (MulAssign , mul_assign , ClosedAddAssign ; const D ; self : Translation < T , D >, right : &'b Translation < T , D >; # [allow (clippy :: suspicious_op_assign_impl)] { self . vector += & right . vector } ; 'b) ;
};
}
