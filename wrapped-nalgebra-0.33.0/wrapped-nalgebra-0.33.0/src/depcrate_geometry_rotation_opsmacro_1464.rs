// Generated macro for macro_1464 (macro)
macro_rules! Depcrate_geometry_rotation_opsmacro_1464 {
() => {
// Module: crate::geometry::rotation_ops
// Provides: {"macro_1464"}
// Dependencies: {}
md_impl_all ! (Mul , mul ; (Const < D1 >, Const < D1 >) , (R2 , C2) const D1 ; for R2 , C2 , SB ; where R2 : Dim , C2 : Dim , SB : Storage < T , R2 , C2 >, DefaultAllocator : Allocator < Const < D1 >, C2 >, ShapeConstraint : AreMultipliable < Const < D1 >, Const < D1 >, R2 , C2 >; self : Rotation < T , D1 >, right : Matrix < T , R2 , C2 , SB >, Output = OMatrix < T , Const < D1 >, C2 >; [val val] => self . into_inner () * right ; [ref val] => self . matrix () * right ; [val ref] => self . into_inner () * right ; [ref ref] => self . matrix () * right ;) ;
};
}
