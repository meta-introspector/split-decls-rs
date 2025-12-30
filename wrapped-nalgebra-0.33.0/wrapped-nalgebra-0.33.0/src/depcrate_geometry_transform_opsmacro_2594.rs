// Generated macro for macro_2594 (macro)
macro_rules! Depcrate_geometry_transform_opsmacro_2594 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"macro_2594"}
// Dependencies: {}
md_assign_impl_all ! (DivAssign , div_assign where T : RealField ; (DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >) , (Const < D >, Const < D >) const D ; for C ; where Const < D >: DimNameAdd < U1 >, C : TCategory , DefaultAllocator : Allocator < DimNameSum < Const < D >, U1 >, DimNameSum < Const < D >, U1 >>; self : Transform < T , C , D >, rhs : Rotation < T , D >; [val] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ; [ref] => # [allow (clippy :: suspicious_op_assign_impl)] { * self *= rhs . inverse () } ;) ;
};
}
