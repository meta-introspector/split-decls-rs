// Generated macro for macro_2579 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2579 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2579"}
// Dependencies: {}
md_impl_all ! (Div , div where T : RealField ; (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) , (Const < D >, Const < D >) const D ; for C ; where Const < D >: DimNameAdd < U1 >, C : TCategoryMul < TAffine >, DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Transform < T , C , D >, rhs : Rotation < T , D >, Output = Transform < T , C :: Representative , D >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * rhs . inverse () } ;) ;
};
}
