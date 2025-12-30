// Generated macro for macro_1466 (macro)
macro_rules! Depcrate_geometry_rotation_opsmacro_1466 {
() => {
// Module: crate::geometry::rotation_ops
// Provides: {"macro_1466"}
// Dependencies: {}
md_impl_all ! (Div , div ; (R1 , C1) , (Const < D2 >, Const < D2 >) const D2 ; for R1 , C1 , SA ; where R1 : Dim , C1 : Dim , SA : Storage < T , R1 , C1 >, DefaultAllocator : Allocator < R1 , Const < D2 >>, ShapeConstraint : AreMultipliable < R1 , C1 , Const < D2 >, Const < D2 >>; self : Matrix < T , R1 , C1 , SA >, right : Rotation < T , D2 >, Output = OMatrix < T , R1 , Const < D2 >>; [val val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref val] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [val ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * right . inverse () } ;) ;
};
}
