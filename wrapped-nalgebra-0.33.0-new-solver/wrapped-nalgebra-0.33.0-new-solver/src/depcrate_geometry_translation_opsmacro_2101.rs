// Generated macro for macro_2101 (macro)
macro_rules! Depcrate_geometry_translation_opsmacro_2101 {
() => {
// Module: crate::geometry::translation_ops
// Provides: {"macro_2101"}
// Dependencies: {}
add_sub_assign_impl ! (MulAssign , mul_assign , ClosedAddAssign ; const D ; self : Translation < T , D >, right : Translation < T , D >; # [allow (clippy :: suspicious_op_assign_impl)] { self . vector += right . vector } ;) ;
};
}
