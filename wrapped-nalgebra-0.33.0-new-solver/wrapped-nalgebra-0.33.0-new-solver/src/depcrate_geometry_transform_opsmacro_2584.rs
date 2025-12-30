// Generated macro for macro_2584 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2584 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2584"}
// Dependencies: {}
md_impl_all ! (Div , div where T : RealField ; (Const < D >, U1) , (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) const D ; for C ; where Const < D >: DimNameAdd < U1 >, C : TCategoryMul < TAffine >, DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Translation < T , D >, rhs : Transform < T , C , D >, Output = Transform < T , C :: Representative , D >; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self . inverse () * rhs } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self . inverse () * rhs } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self . inverse () * rhs } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self . inverse () * rhs } ;) ;
};
}
