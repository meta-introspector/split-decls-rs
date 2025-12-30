// Generated macro for macro_1465 (macro)
macro_rules! Depcrate_geometry_rotation_opsmacro_1465 {
() => {
// Module: crate::geometry::rotation_ops
// Provides: {"macro_1465"}
// Dependencies: {}
md_impl_all ! (Mul , mul ; (R1 , C1) , (Const < D2 >, Const < D2 >) const D2 ; for R1 , C1 , SA ; where R1 : Dim , C1 : Dim , SA : Storage < T , R1 , C1 >, DefaultAllocator : Allocator < R1 , Const < D2 >>, ShapeConstraint : AreMultipliable < R1 , C1 , Const < D2 >, Const < D2 >>; self : Matrix < T , R1 , C1 , SA >, right : Rotation < T , D2 >, Output = OMatrix < T , R1 , Const < D2 >>; [val val] => self * right . into_inner () ; [ref val] => self * right . into_inner () ; [val ref] => self * right . matrix () ; [ref ref] => self * right . matrix () ;) ;
};
}
