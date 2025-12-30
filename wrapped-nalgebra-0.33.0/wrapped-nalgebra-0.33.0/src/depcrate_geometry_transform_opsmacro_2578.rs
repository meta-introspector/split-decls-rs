// Generated macro for macro_2578 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2578 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2578"}
// Dependencies: {}
md_impl_all ! (Div , div where T : RealField ; (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) , (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) const D ; for CA , CB ; where Const < D >: DimNameAdd < U1 >, CA : TCategoryMul < CB >, CB : SubTCategoryOf < TProjective >, DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Transform < T , CA , D >, rhs : Transform < T , CB , D >, Output = Transform < T , CA :: Representative , D >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . clone () . inverse () } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . clone () . inverse () } ;) ;
};
}
