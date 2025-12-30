// Generated macro for macro_2592 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2592 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2592"}
// Dependencies: {}
md_assign_impl_all ! (DivAssign , div_assign where T : RealField ; (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) , (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) const D ; for CA , CB ; where Const < D >: DimNameAdd < U1 >, CA : SuperTCategoryOf < CB >, CB : SubTCategoryOf < TProjective >, DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Transform < T , CA , D >, rhs : Transform < T , CB , D >; [val] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . clone () . inverse () } ;) ;
};
}
